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
