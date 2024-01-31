use ::interpreter::eval;
use ::lexer::*;
use ::object::*;
use ::parser::*;

use compiler::{Compiler, SymbolTable};
use std::cell::RefCell;
use std::io;
use std::io::Write;
use std::rc::Rc;
use vm::VM;

thread_local! {
    static HISTORY: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![]))
}

pub const PROMPT: &'static str = ">> ";
pub const SYMBOL: &'static str = r#">> Monkey Lang 0.1.0 | (Rust BackEnd)"#;

pub fn start() {
    println!("{}", SYMBOL);
    // readline in
    let stdin = io::stdin();
    let mut input = String::new();
    let context = Rc::new(Context::new());
    loop {
        print!("{PROMPT}");
        std::io::stdout().flush().unwrap();
        stdin.read_line(&mut input).unwrap();
        // println!("read key: {:?}", input);
        if input == "" {
            continue;
        }
        HISTORY.with(|history| {
            history.borrow_mut().push(input.clone());
        });
        let lex = Lexer::new(input.clone());
        let p = Parser::new(lex.clone());
        let pr = p.parse_program();
        assert!(pr.is_some());
        if p.errors().borrow().len() != 0 {
            print_parser_errors(p.errors().borrow().as_ref());
            input.clear();
            continue;
        }
        let pr = pr.unwrap();
        // println!("{}", &pr);
        if let Some(r) = eval(&pr, context.clone()).as_ref() {
            println!("{}", r);
        }
        input.clear();
    }
}

pub fn start_with_vm() {
    println!("{}", SYMBOL);
    // readline in
    let stdin = io::stdin();
    let mut input = String::new();

    let mut external_constants: Vec<Rc<dyn Object>> = Compiler::create_constants();
    let mut external_symbol_table: SymbolTable = Compiler::create_symbol_table();
    let mut external_globals: Vec<Rc<dyn Object>> = VM::create_globals();

    loop {
        print!("{PROMPT}");
        std::io::stdout().flush().unwrap();
        stdin.read_line(&mut input).unwrap();
        // println!("read key: {:?}", input);
        if input == "" {
            continue;
        }
        HISTORY.with(|history| {
            history.borrow_mut().push(input.clone());
        });
        let lex = Lexer::new(input.clone());
        let p = Parser::new(lex.clone());
        let pr = p.parse_program();
        assert!(pr.is_some());
        if p.errors().borrow().len() != 0 {
            print_parser_errors(p.errors().borrow().as_ref());
            input.clear();
            continue;
        }

        let pr = pr.unwrap();
        let compi = Compiler::new();
        compi
            .load_external_constants(&mut external_constants)
            .expect("failed to load external constants");
        compi
            .load_external_symbol_table(&mut external_symbol_table)
            .expect("failed to load external symbol table");
        if let Err(e) = compi.compile(&pr) {
            eprintln!("Compile failed {}", e);
            continue;
        }

        let vm = VM::new(compi.bytecode());
        vm.load_external_globals(&mut external_globals)
            .expect("failed to load external globals");
        if let Err(e) = vm.run() {
            eprintln!("VM run failed {}", e);
            continue;
        }

        let stack_top = vm.last_popped_stack_el().expect("get stack top failed");
        println!("{}", stack_top);

        input.clear();
    }
}

pub fn run_with_vm(program: String) {
    let mut input = program.clone();
    let lex = Lexer::new(input.clone());
    let p = Parser::new(lex.clone());
    let pr = p.parse_program();
    assert!(pr.is_some());
    if p.errors().borrow().len() != 0 {
        print_parser_errors(p.errors().borrow().as_ref());
        input.clear();
        return;
    }
    let pr = pr.unwrap();
    let compi = Compiler::new();
    if let Err(e) = compi.compile(&pr) {
        eprintln!("Compile failed {}", e);
    }

    let vm = VM::new(compi.bytecode());
    if let Err(e) = vm.run() {
        eprintln!("VM run failed {}", e);
    }

    let stack_top = vm.last_popped_stack_el().expect("get stack top failed");
    println!("{}", stack_top);
}

pub fn run(program: String) {
    let mut input = program.clone();
    let context = Rc::new(Context::new());
    let lex = Lexer::new(input.clone());
    let p = Parser::new(lex.clone());
    let pr = p.parse_program();
    assert!(pr.is_some());
    if p.errors().borrow().len() != 0 {
        print_parser_errors(p.errors().borrow().as_ref());
        input.clear();
        return;
    }
    let pr = pr.unwrap();
    // dbg!("{}", &pr);
    if let Some(r) = eval(&pr, context.clone()).as_ref() {
        println!("{}", r);
    }
}

pub fn print_parser_errors(errors: &Vec<String>) {
    errors.iter().for_each(|err| {
        eprintln!("\t{}", err);
    });
}
