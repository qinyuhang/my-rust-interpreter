use interpreter::run;

fn main() {
    let input = include_str!("fib.mok");
    run(input.to_string());
}
