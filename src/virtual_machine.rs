mod ast;
mod parser;
mod scanner;
mod token;

use crate::virtual_machine::ast::AST;
use crate::virtual_machine::parser::Parser;
use crate::virtual_machine::token::Token;
use scanner::Scanner;
use std::process::exit;

pub struct VirtualMachine {
    source_code: String,
    source_code_vector: Vec<String>,
    debug: bool,
}

impl VirtualMachine {
    pub fn new(
        source_code: String,
        source_code_vector: Vec<String>,
        debug: bool,
    ) -> VirtualMachine {
        VirtualMachine {
            source_code,
            source_code_vector,
            debug,
        }
    }

    pub fn run(self) {
        // 字句解析
        let scanner: Scanner =
            Scanner::new(self.source_code.clone(), self.source_code_vector.clone());

        let tokens: Vec<Token> = match scanner.scan() {
            Ok(tokens) => tokens,
            Err(error) => {
                eprintln!("{:?}", error.to_string());
                exit(1);
            }
        };

        if self.debug {
            VirtualMachine::print_tokens(tokens.clone());
        }

        let mut parser: Parser = Parser::new(tokens);
        let asts = match parser.generate() {
            Ok(asts) => asts,
            Err(error) => {
                eprintln!("{:?}", error.to_string());
                exit(1);
            }
        };

        if self.debug {
            Self::print_asts(asts.clone());
        }
    }

    fn print_tokens(tokens: Vec<Token>) {
        println!("Scanned Tokens:");
        for token in tokens {
            println!("  {:?}", token.token_type);
        }
        println!("\n");
    }

    fn print_asts(asts: Vec<AST>) {
        println!("AST is created:");
        for (index_of_ast, ast) in asts.into_iter().enumerate() {
            println!("  AST[{}]:", index_of_ast);
            println!("    {:?}", ast);
        }
        println!("\n");
    }
}
