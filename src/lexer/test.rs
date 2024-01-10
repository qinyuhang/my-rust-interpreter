mod test {
    #[allow(unused)]
    use {crate::lexer::Lexer, crate::token};

    #[test]
    fn canary_test() {
        assert_eq!(1, 1);
    }

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";
        let tests = vec![
            (token::ASSIGN, "="),
            (token::PLUS, "+"),
            (token::LPAREN, "("),
            (token::RPAREN, ")"),
            (token::LBRACE, "{"),
            (token::RBRACE, "}"),
            (token::COMMA, ","),
            (token::SEMICOLON, ";"),
            (token::EOF, "\0"),
        ];
        let lex = Lexer::new(input);

        tests.iter().for_each(|test| {
            let p_token = lex.next_token();
            // println!("Running Test: {:?}, lexer.next_token: {:?}", test, p_token);
            assert_eq!(p_token.token_type, test.0);
            assert_eq!(p_token.literal, test.1);
        });
    }

    #[test]
    fn test_next_token_short_code() {
        let input = r#"let five = 5;
let ten = 10;
let add = fn(x, y) {
    x + y;
};
let result = add(five, ten);"#;
        let tests = vec![
            (token::LET, "let"),
            (token::IDENT, "five"),
            (token::ASSIGN, "="),
            (token::INT, "5"),
            (token::SEMICOLON, ";"),
            (token::LET, "let"),
            (token::IDENT, "ten"),
            (token::ASSIGN, "="),
            (token::INT, "10"),
            (token::SEMICOLON, ";"),
            (token::LET, "let"),
            (token::IDENT, "add"),
            (token::ASSIGN, "="),
            (token::FUNCTION, "fn"),
            (token::LPAREN, "("),
            (token::IDENT, "x"),
            (token::COMMA, ","),
            (token::IDENT, "y"),
            (token::RPAREN, ")"),
            (token::LBRACE, "{"),
            (token::IDENT, "x"),
            (token::PLUS, "+"),
            (token::IDENT, "y"),
            (token::SEMICOLON, ";"),
            (token::RBRACE, "}"),
            (token::SEMICOLON, ";"),
            (token::LET, "let"),
            (token::IDENT, "result"),
            (token::ASSIGN, "="),
            (token::IDENT, "add"),
            (token::LPAREN, "("),
            (token::IDENT, "five"),
            (token::COMMA, ","),
            (token::IDENT, "ten"),
            (token::RPAREN, ")"),
            (token::SEMICOLON, ";"),
            (token::EOF, "\0"),
        ];

        let lex = Lexer::new(input);

        tests.iter().for_each(|test| {
            let p_token = lex.next_token();
            // println!("Running Test: {:?}, lexer.next_token: {:?}", test, p_token);
            assert_eq!(p_token.token_type, test.0);
            assert_eq!(p_token.literal, test.1);
        });
    }
    #[test]
    fn test_next_token_short_code_1() {
        let input = r#"let five = 5;
let ten = 10;
let add = fn(x, y) {
    x + y;
};
let result = add(five, ten);
!-/*5;
5 < 10 > 5;
"#;

        let tests = vec![
            (token::LET, "let"),
            (token::IDENT, "five"),
            (token::ASSIGN, "="),
            (token::INT, "5"),
            (token::SEMICOLON, ";"),
            (token::LET, "let"),
            (token::IDENT, "ten"),
            (token::ASSIGN, "="),
            (token::INT, "10"),
            (token::SEMICOLON, ";"),
            (token::LET, "let"),
            (token::IDENT, "add"),
            (token::ASSIGN, "="),
            (token::FUNCTION, "fn"),
            (token::LPAREN, "("),
            (token::IDENT, "x"),
            (token::COMMA, ","),
            (token::IDENT, "y"),
            (token::RPAREN, ")"),
            (token::LBRACE, "{"),
            (token::IDENT, "x"),
            (token::PLUS, "+"),
            (token::IDENT, "y"),
            (token::SEMICOLON, ";"),
            (token::RBRACE, "}"),
            (token::SEMICOLON, ";"),
            (token::LET, "let"),
            (token::IDENT, "result"),
            (token::ASSIGN, "="),
            (token::IDENT, "add"),
            (token::LPAREN, "("),
            (token::IDENT, "five"),
            (token::COMMA, ","),
            (token::IDENT, "ten"),
            (token::RPAREN, ")"),
            (token::SEMICOLON, ";"),
            (token::BANG, "!"),
            (token::MINUS, "-"),
            (token::SLASH, "/"),
            (token::ASTERISK, "*"),
            (token::INT, "5"),
            (token::SEMICOLON, ";"),
            (token::INT, "5"),
            (token::LT, "<"),
            (token::INT, "10"),
            (token::GT, ">"),
            (token::INT, "5"),
            (token::SEMICOLON, ";"),
            (token::EOF, "\0"),
        ];

        let lex = Lexer::new(input);

        tests.iter().for_each(|test| {
            let p_token = lex.next_token();
            // println!("Running Test: {:?}, lexer.next_token: {:?}", test, p_token);
            assert_eq!(p_token.token_type, test.0);
            assert_eq!(p_token.literal, test.1);
        });
    }
    #[test]
    fn test_next_token_short_code_2() {
        let input = r#"let five = 5;
let ten = 10;
let add = fn(x, y) {
    x + y;
};
let result = add(five, ten);
!-/*5;
5 < 10 > 5;
if ( 5 < 10 ) {
    return true;
} else {
    return false;
}

5 == 5;
7 != 5;
"#;

        let tests = vec![
            (token::LET, "let"),
            (token::IDENT, "five"),
            (token::ASSIGN, "="),
            (token::INT, "5"),
            (token::SEMICOLON, ";"),
            (token::LET, "let"),
            (token::IDENT, "ten"),
            (token::ASSIGN, "="),
            (token::INT, "10"),
            (token::SEMICOLON, ";"),
            (token::LET, "let"),
            (token::IDENT, "add"),
            (token::ASSIGN, "="),
            (token::FUNCTION, "fn"),
            (token::LPAREN, "("),
            (token::IDENT, "x"),
            (token::COMMA, ","),
            (token::IDENT, "y"),
            (token::RPAREN, ")"),
            (token::LBRACE, "{"),
            (token::IDENT, "x"),
            (token::PLUS, "+"),
            (token::IDENT, "y"),
            (token::SEMICOLON, ";"),
            (token::RBRACE, "}"),
            (token::SEMICOLON, ";"),
            (token::LET, "let"),
            (token::IDENT, "result"),
            (token::ASSIGN, "="),
            (token::IDENT, "add"),
            (token::LPAREN, "("),
            (token::IDENT, "five"),
            (token::COMMA, ","),
            (token::IDENT, "ten"),
            (token::RPAREN, ")"),
            (token::SEMICOLON, ";"),
            (token::BANG, "!"),
            (token::MINUS, "-"),
            (token::SLASH, "/"),
            (token::ASTERISK, "*"),
            (token::INT, "5"),
            (token::SEMICOLON, ";"),
            (token::INT, "5"),
            (token::LT, "<"),
            (token::INT, "10"),
            (token::GT, ">"),
            (token::INT, "5"),
            (token::SEMICOLON, ";"),
            (token::IF, "if"),
            (token::LPAREN, "("),
            (token::INT, "5"),
            (token::LT, "<"),
            (token::INT, "10"),
            (token::RPAREN, ")"),
            (token::LBRACE, "{"),
            (token::RETURN, "return"),
            (token::TRUE, "true"),
            (token::SEMICOLON, ";"),
            (token::RBRACE, "}"),
            (token::ELSE, "else"),
            (token::LBRACE, "{"),
            (token::RETURN, "return"),
            (token::FALSE, "false"),
            (token::SEMICOLON, ";"),
            (token::RBRACE, "}"),
            //
            (token::INT, "5"),
            (token::EQ, "=="),
            (token::INT, "5"),
            (token::SEMICOLON, ";"),
            //
            (token::INT, "7"),
            (token::NOT_EQ, "!="),
            (token::INT, "5"),
            (token::SEMICOLON, ";"),
            (token::EOF, "\0"),
        ];

        let lex = Lexer::new(input);

        tests.iter().for_each(|test| {
            let p_token = lex.next_token();
            // println!("Running Test: {:?}, lexer.next_token: {:?}", test, p_token);
            assert_eq!(p_token.token_type, test.0);
            assert_eq!(p_token.literal, test.1);
        });
    }
    #[test]
    fn test_short() {
        let input = r#"if (a > 10)"#;
        let lex = Lexer::new(input);

        let mut tk = lex.next_token();
        let mut count = 0;
        while tk.token_type != token::EOF {
            count += 1;
            println!("{:?}", tk);
            tk = lex.next_token();
        }
        assert_eq!(count, 6);
    }

    #[test]
    fn test_hex_binary_string() {
        let input = "0x01; 0b1";
        let lex = Lexer::new(input);

        let mut tk = lex.next_token();
        let mut count = 0;
        while tk.token_type != token::EOF {
            count += 1;
            println!("{:?}", tk);
            tk = lex.next_token();
        }
        assert_eq!(count, 3);
    }

    #[test]
    fn test_string_literal() {
        let input = r#""foobar""#;

        let tests = vec![
            (token::STRING, "foobar"),
            (token::EOF, "\0"),
        ];

        let lex = Lexer::new(input);

        tests.iter().for_each(|test| {
            let p_token = lex.next_token();
            // println!("Running Test: {:?}, lexer.next_token: {:?}", test, p_token);
            assert_eq!(p_token.token_type, test.0);
            assert_eq!(p_token.literal, test.1);
        });
    }
//     #[test]
//     fn test_unicode() {
//         let input = r#"let abcd = 1;
// let 中文名字 = 1;"#;
//         let lex = Lexer::new(input);
//         let mut tk = lex.next_token();
//         let mut count = 0;
//         while tk.token_type != token::EOF {
//             count += 1;
//             println!("{:?}", tk);
//             tk = lex.next_token();
//         }
//         // FIXME: 等待 is_letter 方法支持unicode更多字符
//         assert_eq!(count, 10);
//     }
}
