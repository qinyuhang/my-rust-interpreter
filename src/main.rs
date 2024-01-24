// TODO run will run args file
// without run is repl
// eg:
// `mki run -i <file>.mok` will run <file>.mok
// `mki` will start repl
// `mik compile -i <file>.mok -o <out_file>.mokb` will compile <file>.mok to <out_file>.mokb
use clap::{Parser, Subcommand};
use interpreter::{run, start};
use std::path::PathBuf;

#[derive(Subcommand)]
enum Commands {
    Run {
        file: PathBuf,
    },
    Compile {
        file: PathBuf,
        #[arg(short, long)]
        out: PathBuf,
    },
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Run { file }) => {
            // dbg!("{}", file);
            if let Ok(file) = std::fs::read_to_string(file) {
                run(file);
            } else {
                eprintln!("file not found {:?}", file);
            }
        }
        Some(Commands::Compile { file, out: _ }) => {
            if let Ok(_file) = std::fs::read_to_string(file) {
                // repl::run(file);
            } else {
                eprintln!("file not found {:?}", file);
            }
        }
        _ => {
            // let cargo_file = include_str!("../Cargo.toml");
            // println!("Hello, world! {}", cargo_file);
            start();
        }
    };
}
