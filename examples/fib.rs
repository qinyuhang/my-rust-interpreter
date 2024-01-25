fn fib(n: i32) -> i32 {
    if n == 0 {
        return 1;
    } else {
        if n == 1 {
            return 1;
        } else {
            return fib(n - 1) + fib(n - 2);
        }
    }
}

fn main() {
    // assert_eq!(fib(0), 1);
    // assert_eq!(fib(1), 1);
    // assert_eq!(fib(2), 2);
    // assert_eq!(fib(3), 3);
    // assert_eq!(fib(4), 5);
    dbg!(fib(30));
}
