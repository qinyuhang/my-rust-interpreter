use crate::lexer::*;
use crate::parser::*;
use std::io;

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
  xx          xxxx
xxxxxx       xxxxxxx"#;

pub fn start() {
    // readline in
    let stdin = io::stdin();
    let mut input = String::new();
    loop {
        println!("{PROMPT}");
        stdin.read_line(&mut input).unwrap();
        let lex = Lexer::new(input.clone());
        let p = Parser::new(lex.clone());
        let pr = p.parse_program();
        assert!(pr.is_some());
        if p.errors().borrow().len() != 0 {
            println!("{}\n", SYMBOL);
            print_parser_errors(p.errors().borrow().as_ref());
            continue;
        }
        let pr = pr.unwrap();
        println!("{}", SYMBOL);
        println!("\n\nParsed Program:\n{}", &pr);
        // loop {
        //     #[allow(unused_mut)]
        //     let mut tok = lex.next_token();
        //     println!("tok: {:?}", tok);
        //     if tok.token_type == EOF {
        //         break;
        //     }
        // }
        // println!("{input}");
        input.clear();
        // print!("\r{PROMPT}");
    }

    // for line in stdin.lock().lines() {
    //     let line = line.unwrap();
    //     if line == "" {
    //         continue;
    //     }
    //     print!("{PROMPT}");
    //     let lex = Lexer::new(line);

    //     loop {
    //         let mut tok = lex.next_token();
    //         println!("{:?}", tok);
    //         if tok.Type == EOF {
    //             break;
    //         }

    //         tok = lex.next_token();
    //     }
    // }
}

pub fn print_parser_errors(errors: &Vec<String>) {
    errors.iter().for_each(|err| {
        println!("\t{}", err);
    });
}