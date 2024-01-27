use interpreter::run;

fn main() {
    let input = include_str!("fib_loop.mok");
    run(input.to_string());
}
