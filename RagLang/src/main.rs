mod ast;
mod lexer;
mod parser;
mod token;
mod interpreter;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::token::Token;
use crate::interpreter::Interpreter;

fn main() {
    // let input = r#"
    //     let x = 3;
    //     while x {
    //         print x;
    //         let x = x - 1;
    //     }
    // "#;

    //Nested if test
    // let input = r#"
    //     let x = 2;
    //     if x > 0 {
    //         print x;
    //         if x == 2 {
    //             print 100;
    //         } else {
    //             print 200;
    //         }
    //         let x = x - 1;
    //     } else {
    //         print 0;
    //     }
    //     print x;
    // "#;

    // //If in a loop test
    // let input = r#"
    //     let x = 10;
    //     while x > 0 {
    //         if x >= 5 {
    //             print x;
    //         } else {
    //             print 0;
    //         }
    //         let x = x - 1;
    //     }
    // "#;

    //Two vars test
    let input = r#"
        let x = 10;
        let y = x;
        if x > y {
            print x;
        } else {
            print y;
        }
        print 0;
    "#;
   

    // Lex input into tokens
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();
    loop {
        let tok = lexer.next_token();
        tokens.push(tok.clone());
        if tok == Token::EOF {
            break;
        }
    }

    // Parse tokens into AST
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program();

    // Print the AST (optional)
    println!("Parsed AST:");
    for stmt in &program {
        println!("{:#?}", stmt);
    }

    // Create interpreter and run the program
    let mut interpreter = Interpreter::new();
    interpreter.exec_program(program);
}
