pub use my_rust_interpreter::repl;

fn main() {
    // let cargo_file = include_str!("../Cargo.toml");
    // println!("Hello, world! {}", cargo_file);
    repl::start();
}
