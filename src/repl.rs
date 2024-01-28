use ::interpreter::eval;
use ::lexer::*;
use ::object::*;
use ::parser::*;

use compiler::Compiler;
use std::cell::RefCell;
use std::io;
use std::io::Write;
use std::rc::Rc;
use vm::VM;

thread_local! {
    static HISTORY: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![]))
}

pub const PROMPT: &'static str = ">> ";
pub const SYMBOL: &'static str = r#"
 xxxxxxxxxxxx
  xx       xxx
  xx        xxx
  xx        xxx
  xx       xxx
  xxxxxxxxxxx
  xxxx
  xxxxxx
  xx   xxx
  xx    xxxx
  xx      xxxx
  xx        xxxx
  xx          xxxx  #
######       ########"#;

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
        if let Err(e) = compi.compile(&pr) {
            eprintln!("Compile failed {}", e);
            continue;
        }

        let vm = VM::new(compi.bytecode());
        if let Err(e) = vm.run() {
            eprintln!("VM run failed {}", e);
            continue;
        }

        let stack_top = vm.stack_top().expect("get stack top failed");
        println!("{}", stack_top);

        input.clear();
    }
}

pub fn run_with_vm(program: String) {
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
    let compi = Compiler::new();
    if let Err(e) = compi.compile(&pr) {
        eprintln!("Compile failed {}", e);
    }

    let vm = VM::new(compi.bytecode());
    if let Err(e) = vm.run() {
        eprintln!("VM run failed {}", e);
    }

    let stack_top = vm.stack_top().expect("get stack top failed");
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
