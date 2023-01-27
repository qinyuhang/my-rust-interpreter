mod ast;
mod lexer;
mod parser;
mod repl;
mod token;
mod utils;
mod evaluator;
mod object;

fn main() {
    let cargo_file = include_str!("../Cargo.toml");
    println!("Hello, world! {}", cargo_file);
    repl::start();
}
