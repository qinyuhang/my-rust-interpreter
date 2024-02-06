use ast::Identifier;
use std::cell::Cell;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub type SymbolScope = &'static str;

pub const GLOBAL_SCOPE: SymbolScope = "GLOBAL";
pub const LOCAL_SCOPE: SymbolScope = "LOCAL";
pub const BUILTIN_SCOPE: SymbolScope = "BUILTIN";
pub const FREE_SCOPE: SymbolScope = "FREE";

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Symbol {
    pub name: Rc<Identifier>,
    pub scope: SymbolScope,
    pub index: usize, // or other int?
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct SymbolTable {
    pub outer: RefCell<Option<Rc<SymbolTable>>>,
    pub free_symbols: RefCell<Vec<Rc<Symbol>>>,
    store: RefCell<HashMap<Rc<Identifier>, Rc<Symbol>>>,
    num_definitions: Cell<usize>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            outer: RefCell::new(None),
            free_symbols: RefCell::new(vec![]),
            store: Default::default(),
            num_definitions: Default::default(),
        }
    }
    pub fn define(&self, name: Rc<Identifier>) -> Rc<Symbol> {
        let scope = self
            .outer
            .borrow()
            .as_ref()
            .map_or_else(|| GLOBAL_SCOPE, |_| LOCAL_SCOPE);

        let symbol = Rc::new(Symbol {
            name: name.clone(),
            index: self.num_definitions.get(),
            scope,
        });
        self.store.borrow_mut().insert(name, symbol.clone());
        self.num_definitions.set(self.num_definitions.get() + 1);
        symbol
    }

    pub fn define_free(&self, origin: Rc<Symbol>) -> Rc<Symbol> {
        self.free_symbols.borrow_mut().push(origin.clone());

        let name = origin.name.clone();
        let symbol = Rc::new(Symbol {
            name: name.clone(),
            index: self.free_symbols.borrow().len() - 1,
            scope: FREE_SCOPE,
        });
        // insert twice will replace the original one
        self.store.borrow_mut().insert(name, symbol.clone());
        symbol
    }

    pub fn resolve(&self, name: Rc<Identifier>) -> Result<Rc<Symbol>, String> {
        let r = self.store.borrow().get(&name).cloned().map(|v| v.clone());
        if let Some(r) = r {
            return Ok(r);
        }
        let r = self
            .outer
            .borrow()
            .as_ref()
            .ok_or_else(|| format!("Fail get {}", &name))
            .and_then(|outer| outer.resolve(name))?;
        if r.scope == GLOBAL_SCOPE || r.scope == BUILTIN_SCOPE {
            return Ok(r);
        }
        Ok(self.define_free(r))
    }

    pub fn define_count(&self) -> usize {
        self.num_definitions.get()
    }

    pub fn new_enclosed(outer: Rc<SymbolTable>) -> Self {
        let r = Self::new();
        r.outer.borrow_mut().replace(outer);
        r
    }

    pub fn define_builtin(&self, index: usize, name: Rc<Identifier>) -> Rc<Symbol> {
        let symbol = Rc::new(Symbol {
            name: name.clone(),
            index,
            scope: BUILTIN_SCOPE,
        });
        self.store.borrow_mut().insert(name, symbol.clone());
        symbol
    }
}
