use crate::{testing_result, TestingResult};

// FIXME: change to? macros
thread_local! {
    pub static BUILTIN_CASES: Vec<(&'static str, TestingResult)> = vec![
        (r#"len("")"#, testing_result!(Int, 0)),
        (r#"len("four")"#, testing_result!(Int, 4)),
        (r#"len("hello world")"#, testing_result!(Int, 11)),
        (
            r#"len(1) "#,
            testing_result!(Err, "argument to `len` not supported, got INTEGER"),
        ),
        (
            r#"len("one", "two")"#,
            testing_result!(Err, "wrong number of arguments. got=2, want=1"),
        ),
        (r#"len([12, 2,3])"#, testing_result!(Int, 3)),
        (r#"len([])"#, testing_result!(Int, 0)),
        (r#"puts("hello", "world!")"#, testing_result!(Nil)),
        (r#"first([1,2,31])"#, testing_result!(Int, 1)),
        (r#"first([])"#, testing_result!(Nil)),
        (
            r#"first(1)"#,
            testing_result!(Err, "argument to `first` must be ARRAY, got INTEGER"),
        ),
        (r#"last([1, 2, 3])"#, testing_result!(Int, 3)),
        (r#"last([])"#, testing_result!(Nil)),
        (
            r#"last(1)"#,
            testing_result!(Err, "argument to `last` must be ARRAY, got INTEGER"),
        ),
        (r#"rest([1, 2, 3])"#, testing_result!(Vec, vec![2, 3])),
        // !attention different from the book, I believe rest should always return a new array
        (r#"rest([])"#, testing_result!(Vec, vec![])), // Panic
        (r#"push([], 1)"#, testing_result!(Vec, vec![1])),
        (
            r#"push(1, 1)"#,
            testing_result!(Err, "argument[0] to `push` must be ARRAY, got INTEGER"),
        ),
    ];
}
thread_local! {
    pub static STRING_LITERAL_CASE: Vec<(&'static str, TestingResult)> = vec![
        (r#""""#, testing_result!(String, "")),
        (r#""foobar""#, testing_result!(String, "foobar")),
    ];
}

thread_local! {
    pub static CLOSURE_CASE: Vec<(&'static str, TestingResult)> = vec![
        (r#"
let newClosure = fn(a) { fn() { a } };
let closure = newClosure(99);
closure()"#, testing_result!(Int, 99)),
        (r#"
let newAddr = fn(a,b) { fn(c) { a + b + c }};
let addr = newAddr(1,2);
addr(8)"#, testing_result!(Int, 11)),
        (r#"
let newAddr = fn(a,b) {
  let c = a + b;
  fn(d) { c + d }};
let addr = newAddr(1,2);
addr(8)"#, testing_result!(Int, 11)),
        (r#"
let newAdderOuter = fn(a, b) {
    let c = a + b;
    fn(d) {
        let e = d + c;
        fn(f) { e + f; };
    }
};
let newAdderInner = newAdderOuter(1, 2);
let adder = newAdderInner(3);
adder(8);
"#, testing_result!(Int, 14)),
    (r#"
let a = 1;
let newAdderOuter = fn(b) {
     fn(c) {
         fn(d) { a + b + c + d };
}; };
let newAdderInner = newAdderOuter(2);
let adder = newAdderInner(3);
adder(8);
"#, testing_result!(Int, 14)),
    (r#"
let newClosure = fn(a, b) {
    let one = fn() { a; };
    let two = fn() { b; };
    fn() { one() + two(); };
};
let closure = newClosure(9, 90);
closure();"#, testing_result!(Int, 99)),
    ];
}

thread_local! {
    pub static WHILE_CASE: Vec<(&'static str, TestingResult)> = vec![
        (
                r#"let a = 0; while (a < 1) { a += 1; break; }; a;"#,
                testing_result!(Int, 1),
            ),
            (
                r#"let a = 0; while (a < 1) { break; }; a;"#,
                testing_result!(Int, 0),
            ),
            (
                r#"
let a = 0;
let b = fn() {
  while (a < 5) { a += 1; };
  return a;
};
b();
            "#,
                testing_result!(Int, 5),
            ),
    ];
}

thread_local! {
    pub static UPDATE_CASE: Vec<(&'static str, TestingResult)> = vec![
            (r#"let a = 0; a += 1; a;"#, testing_result!(Int, 1)),
            (
                r#"let a = 0; if (true) { a += 1;}; a;"#,
                testing_result!(Int, 1),
            ),
            (
                r#"let a = 0; let b = fn() { a += 1;}; b(); a;"#,
                testing_result!(Int, 1),
            ),
            (r#"let a = 0; a -= 1; a;"#, testing_result!(Int, -1)),
            (
                r#"let a = 0; if (true) { a -= 1;}; a;"#,
                testing_result!(Int, -1),
            ),
            (
                r#"let a = 0; let b = fn() { a -= 1;}; b(); a;"#,
                testing_result!(Int, -1),
            ),
            (r#"let a = 2; a *= 5; a;"#, testing_result!(Int, 10)),
            (
                r#"let a = 2; if (true) { a *= 5;}; a;"#,
                testing_result!(Int, 10),
            ),
            (
                r#"let a = 2; let b = fn() { a *= 5;}; b(); a;"#,
                testing_result!(Int, 10),
            ),
            (r#"let a = 10; a /= 2; a;"#, testing_result!(Int, 5)),
            (
                r#"let a = 10; if (true) { a /= 2;}; a;"#,
                testing_result!(Int, 5),
            ),
            (
                r#"let a = 10; let b = fn() { a /= 2;}; b(); a;"#,
                testing_result!(Int, 5),
            ),
        ];
}

thread_local! {
    pub static ASSIGN_CASE: Vec<(&'static str, TestingResult)> = vec![
        (r#"let a = 0; a = 1; a;"#, testing_result!(Int, 1)),
        (
            r#"let a = 0; if (true) { a = 1;}; a;"#,
            testing_result!(Int, 1),
        ),
        (
            r#"let a = 0; let b = fn() { a = 1;}; b(); a;"#,
            testing_result!(Int, 1),
        ),
    ];
}

thread_local! {
    pub static FLOAT_CASE: Vec<(&'static str, TestingResult)> = vec![
        (r#"3.14"#, testing_result!(Float, 3.14)),
        (r#"3.0 + 5.1"#, testing_result!(Float, 8.1)),
        (r#"3.0 - 5.1"#, testing_result!(Float, 3.0 - 5.1)),
        (r#"3.0 * 5.1"#, testing_result!(Float, 3.0 * 5.1)),
        (r#"3.0 / 5.1"#, testing_result!(Float, 3.0 / 5.1)),
        (r#"3 + 5.1"#, testing_result!(Float, 3f64 + 5.1)),
        (r#"3 - 5.1"#, testing_result!(Float, 3f64 - 5.1)),
        (r#"3 * 5.1"#, testing_result!(Float, 3f64 * 5.1)),
        (r#"3 / 5.1"#, testing_result!(Float, 3f64 / 5.1)),
    ]
}

thread_local! {
    pub static ADD_RETURN_VALUE_CASE: Vec<(&'static str, TestingResult)> = vec![
        (r#"fn() { true }() == true;"#, testing_result!(Bool, true)),
        (r#"fn() { 5 }() == 5;"#, testing_result!(Bool, true)),
        (r#"fn() { 5 }() + 5;"#, testing_result!(Int, 10)),
        (r#"fn() { 5 }() - 5;"#, testing_result!(Int, 0)),
        (r#"fn() { 5 }() * 5;"#, testing_result!(Int, 25)),
        (r#"fn() { 5 }() / 5;"#, testing_result!(Int, 1)),
    ];
}

thread_local! {
    pub static HASH_INDEX_CASE: Vec<(&'static str, TestingResult)> = vec![
        (r#"{"one": 1}["one"]"#, testing_result!(Int, 1)),
        (r#"{"foo": 5}["bar"]"#, testing_result!(Nil)),
        (
            r#"let key = "foo"; {"foo": 5}[key]"#,
            testing_result!(Int, 5),
        ),
        (r#"{}["foo"]"#, testing_result!(Nil)),
        (r#"{5:5}[5]"#, testing_result!(Int, 5)),
        (r#"{true: 5}[true]"#, testing_result!(Int, 5)),
        (r#"{false: 5}[false]"#, testing_result!(Int, 5)),
        (
            r#"{"name": 5}[fn(x) {x} ]"#,
            testing_result!(Err, "unusable as hash key: FUNCTION_OBJECT"),
        ),
    ];
}

thread_local! {
    pub static HASH_EVAL_CASE: Vec<(&'static str, TestingResult)> = vec![
        ("{}", testing_result!(Hash, std::collections::HashMap::new())),
        (r#"{"one": 1}"#, testing_result!(Hash, std::collections::HashMap::new())),
        (
            r#"{"one": 1, "two": 1 + 1}"#,
            testing_result!(Hash, std::collections::HashMap::new()),
        ),
        (
            r#"{"on" + "e": 1, "two": 1 + 1}"#,
            testing_result!(Hash, std::collections::HashMap::new()),
        ),
    ];
}

thread_local! {
    pub static COMPOSE_BUILTIN_ARRAY_FN_CASE: Vec<(&'static str, TestingResult)> = vec![
        (
            r#"
    let map = fn(arr, f) {
        let iter = fn(arr, acc) {
            if (len(arr) == 0) {
                acc
            } else {
                iter(rest(arr), push(acc, f(first(arr))));
            }
        };

        iter(arr, []);
    };

    let a = [1,2,3];
    map(a, fn(x) { x * 2 })
"#,
            testing_result!(Vec, vec![2, 4, 6]),
        ),
        (
            r#"
        let reduce = fn(arr, initial, f) {
        let iter = fn(arr, result) {
            if (len(arr) == 0) {
                result
            } else {
                iter(rest(arr), f(result, first(arr)));
            }
        };

        iter(arr, initial);

    };

    let a = [1,2,3];
    reduce(a, 0, fn(x,y) { x + y })"#,
            testing_result!(Int, 6),
        ),
        (
            r#"
        let reduce = fn(arr, initial, f) {
        let iter = fn(arr, result) {
            if (len(arr) == 0) {
                result
            } else {
                iter(rest(arr), f(result, first(arr)));
            }
        };

        iter(arr, initial);

    };

    let sum = fn(arr) {
        reduce(arr, 0, fn(x, y) { x + y })
    };

    let a = [1,2,3];
    sum(a)"#,
            testing_result!(Int, 6),
        ),
    ];
}

thread_local! {
    pub static INDEX_EXPRESSION_CASE: Vec<(&'static str, TestingResult)> = vec![
        ("[1,2,3][0]", testing_result!(Int, 1)),
        ("[1,2,3][1]", testing_result!(Int, 2)),
        ("[1,2,3][2]", testing_result!(Int, 3)),
        ("let i = 0; [1][i]", testing_result!(Int, 1)),
        ("[1,2,3][1+1]", testing_result!(Int, 3)),
        ("let arr=[1,2,3]; arr[2]", testing_result!(Int, 3)),
        (
            "let arr=[1,2,3]; arr[0] + arr[1] + arr[2]",
            testing_result!(Int, 6),
        ),
        ("[1,2,3][3]", TestingResult::Nil),
        ("[1,2,3][-1]", TestingResult::Nil),
    ];
}

thread_local! {
    pub static ARRAY_LITERAL_CASE: Vec<(&'static str, TestingResult)> = vec![
        ("[1,2,3];", testing_result!(Vec, vec![1, 2, 3])),
        ("[1,2+1,3];", testing_result!(Vec, vec![1, 3, 3])),
        ("[1,2+5,3];", testing_result!(Vec, vec![1, 7, 3])),
        (
            "let a = fn() { 5 }; [1,a(),3];",
            testing_result!(Vec, vec![1, 5, 3]),
        ),
        (
            "let b = 5; let a = fn() { b }; [1,a(),3];",
            testing_result!(Vec, vec![1, 5, 3]),
        ),
    ];
}

thread_local! {
    pub static RECUSIVE_CASE: Vec<(&'static str, TestingResult)> = vec![
        (r#"
let countDown = fn(x) {
    if (x == 0) { return 0; }
    else { countDown(x - 1) }
};
countDown(1);
"#, testing_result!(Int, 0)),
        (r#"
let countDown = fn(x) {
    if (x == 0) { return 0; }
    else { countDown(x - 1) }
};
let wrapper = fn() {
    countDown(1);
};
wrapper();
"#, testing_result!(Int, 0)),
        (r#"
let wrapper = fn() {
    let countDown = fn(x) {
        if (x == 0) { return 0; }
        else { countDown(x - 1) }
    };
    countDown(1)
};
wrapper();
"#, testing_result!(Int, 0)),
    ];
}
