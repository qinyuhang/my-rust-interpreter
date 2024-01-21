// TODO run will run args file
// without run is repl
// eg:
// `mki run -i <file>.mok` will run <file>.mok
// `mki` will start repl
// `mik compile -i <file>.mok -o <out_file>.mokb` will compile <file>.mok to <out_file>.mokb

use clap::*;
pub use my_rust_interpreter::repl;

fn main() {
    // let cargo_file = include_str!("../Cargo.toml");
    // println!("Hello, world! {}", cargo_file);
    repl::start();
}
