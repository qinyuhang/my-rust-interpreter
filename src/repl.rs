use crate::lexer::*;
use crate::parser::Parser;
use crate::token::*;
use std::io;

pub const PROMPT: &'static str = ">> ";

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
        let pr = pr.unwrap();
        println!("{}", &pr);
        loop {
            #[allow(unused_mut)]
            let mut tok = lex.next_token();
            println!("{:?}", tok);
            if tok.token_type == EOF {
                break;
            }
        }
        println!("{input}");
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
