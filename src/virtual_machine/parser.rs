mod core;
mod declaration_parser;
mod expression_parser;
mod parser_error;
mod statement_parser;

use crate::virtual_machine::ast::{Statement, AST};
use crate::virtual_machine::token::token_type::TokenType;
use crate::virtual_machine::token::Token;
use parser_error::ParserError;
use statement_parser::parse_statement;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<AST>, ParserError> {
        let mut asts: Vec<AST> = Vec::new();

        while !self.check(TokenType::Eof) {
            let line: usize = self.peek().line;
            let statement: Statement = parse_statement(self)?;

            asts.push(AST::new(line, statement));
        }

        Ok(asts)
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn peek_next(&self) -> &Token {
        &self.tokens[self.current + 1]
    }

    fn advance(&mut self) -> &Token {
        self.current += 1;
        &self.tokens[self.current - 1]
    }

    fn check(&self, token_type: TokenType) -> bool {
        self.peek().token_type == token_type
    }

    pub fn check_advance(&mut self, token_type: TokenType) -> Result<(), ParserError> {
        if self.check(token_type.clone()) {
            self.advance();
            Ok(())
        } else {
            Err(ParserError::MismatchedToken {
                expected: token_type,
                found: self.peek().token_type.clone(),
                line: self.peek().line,
                char_pos: self.peek().char_pos,
            })
        }
    }

    fn match_token(&mut self, token_type: TokenType) -> bool {
        if self.check(token_type.clone()) {
            self.advance();
            true
        } else {
            false
        }
    }
}
