use crate::object::*;
use crate::Identifier;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Context {
    pub scope: RefCell<HashMap<Identifier, Rc<dyn Object>>>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            scope: RefCell::new(HashMap::new()),
        }
    }
    pub fn set(&self, name: Identifier, val: Rc<dyn Object>) {
        self.scope.borrow_mut().insert(name, val);
    }
    pub fn get(&self, name: &Identifier) -> Option<Rc<dyn Object>> {
        self.scope.borrow().get(name).cloned()
    }
    pub fn extend(parent: &Self) -> Self {
        let x = parent.scope.borrow().clone();

        Context {
            scope: RefCell::new(x),
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

        let context1 = Context::extend(&context);
        assert_eq!(context1.scope.borrow().len(), 1);
        let c1 = genId("c1");
        context1.set(
            c1,
            Rc::new(ErrorObject {
                message: "for_context1".into(),
            }),
        );
        assert_eq!(context1.scope.borrow().len(), 2);
        assert_eq!(context.scope.borrow().len(), 1);

        let c = genId("c");
        context.set(
            c,
            Rc::new(ErrorObject {
                message: "for_context".into(),
            }),
        );
        assert_eq!(context1.scope.borrow().len(), 2);
        assert_eq!(context.scope.borrow().len(), 2);
    }
}
