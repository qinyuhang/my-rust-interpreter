use crate::object::*;
use crate::Identifier;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct Context {
    pub parent: Option<Rc<Context>>,
    pub scope: RefCell<HashMap<Identifier, Rc<dyn Object>>>,
}

impl Context {
    pub fn default() -> Self {
        Context {
            scope: RefCell::new(HashMap::new()),
            parent: None,
        }
    }
    pub fn new() -> Self {
        Context {
            scope: RefCell::new(HashMap::new()),
            parent: None,
        }
    }
    pub fn set(&self, name: Identifier, val: Rc<dyn Object>) {
        self.scope.borrow_mut().insert(name, val);
    }
    pub fn get(&self, name: &Identifier) -> Option<Rc<dyn Object>> {
        if let Some(current) = self.scope.borrow().get(name).cloned() {
            return Some(current);
        }
        if let Some(ref parent) = self.parent {
            return parent.get(name);
        }
        None
    }
    pub fn extend(parent: Rc<Self>) -> Self {
        Context {
            scope: RefCell::new(HashMap::new()),
            parent: Some(parent.clone()),
        }
    }
}

mod test {
    use crate::*;

    #[test]
    fn test_new_context() {
        let context = Context::new();
        assert_eq!(context.scope.borrow().len(), 0);
    }

    fn genId(name: &str) -> Identifier {
        Identifier {
            token: Token {
                token_type: IDENT,
                literal: name.to_string(),
            },
            value: name.to_string(),
        }
    }

    #[test]
    fn test_insert() {
        let context = Context::new();
        assert_eq!(context.scope.borrow().len(), 0);
        let key = genId("foobar");
        context.set(
            key.clone(),
            Rc::new(ErrorObject {
                message: "error".into(),
            }),
        );
        assert_eq!(context.scope.borrow().len(), 1);
        let r = context.get(&key);
        assert!(r.is_some());
    }

    #[test]
    fn test_extend() {
        let context = Rc::new(Context::new());
        assert_eq!(context.scope.borrow().len(), 0);
        let key = genId("foobar");
        context.set(
            key.clone(),
            Rc::new(ErrorObject {
                message: "error".into(),
            }),
        );
        assert_eq!(context.scope.borrow().len(), 1);
        let r = context.get(&key);
        assert!(r.is_some());

        let context1 = Context::extend(context.clone());

        assert_eq!(context1.scope.borrow().len(), 0);
        assert!(context1.get(&key.clone()).is_some());

        let c1 = genId("c1");
        context1.set(
            c1,
            Rc::new(ErrorObject {
                message: "for_context1".into(),
            }),
        );
        assert_eq!(context1.scope.borrow().len(), 1);
        assert_eq!(context.scope.borrow().len(), 1);

        let c = genId("c");
        context.set(
            c.clone(),
            Rc::new(ErrorObject {
                message: "for_context".into(),
            }),
        );
        assert_eq!(context1.scope.borrow().len(), 1);
        assert_eq!(context.scope.borrow().len(), 2);
        assert!(context1.get(&c.clone()).is_some());
    }
}
