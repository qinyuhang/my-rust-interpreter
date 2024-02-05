use my_rust_interpreter::repl::run_with_vm;

fn main() {
    let input = include_str!("fib.mok");
    run_with_vm(input.to_string());
}
