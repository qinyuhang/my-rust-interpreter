fn fib(n: i32) -> i32 {
    let mut a = 0;
    let mut b = 1;
    let mut temp;

    for _ in 0..n {
        temp = a + b;
        a = b;
        b = temp;
    }

    b
}

fn main() {
    // assert_eq!(fib(0), 1);
    // assert_eq!(fib(1), 1);
    // assert_eq!(fib(2), 2);
    // assert_eq!(fib(3), 3);
    // assert_eq!(fib(4), 5);
    dbg!(fib(35));
}
