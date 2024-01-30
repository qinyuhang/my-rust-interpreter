use ast::Identifier;
use std::cell::Cell;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub type SymbolScope = &'static str;

pub const GLOBAL_SCOPE: SymbolScope = "GLOBAL";

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Symbol {
    pub name: Rc<Identifier>,
    pub scope: SymbolScope,
    pub index: usize, // or other int?
}

#[derive(Debug, Default)]
pub struct SymbolTable {
    store: RefCell<HashMap<Rc<Identifier>, Rc<Symbol>>>,
    num_definitions: Cell<usize>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            store: Default::default(),
            num_definitions: Default::default(),
        }
    }
    pub fn define(&self, name: Rc<Identifier>) -> Rc<Symbol> {
        let symbol = Rc::new(Symbol {
            name: name.clone(),
            index: self.num_definitions.get(),
            scope: GLOBAL_SCOPE,
        });
        self.store.borrow_mut().insert(name, symbol.clone());
        self.num_definitions.set(self.num_definitions.get() + 1);
        symbol
    }

    pub fn resolve(&self, name: Rc<Identifier>) -> Result<Rc<Symbol>, String> {
        self.store
            .borrow()
            .get(&name)
            .map(|v| v.clone())
            .ok_or(format!("Fail get {}", &name))
    }
}
