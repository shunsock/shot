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

    /// 現在のトークンを見る
    ///
    /// # Returns
    ///
    /// * &Token - 現在のトークン
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    /// 次のトークンを確認する
    ///
    /// # Returns
    ///
    /// * &Token - 次のトークン
    fn peek_next(&self) -> &Token {
        &self.tokens[self.current + 1]
    }

    /// 次のトークンに進む
    ///
    /// # Returns
    ///
    /// * &Token - 進めた後の時点で、現在のトークン
    fn advance(&mut self) -> &Token {
        self.current += 1;
        &self.tokens[self.current - 1]
    }

    /// 次のトークンが指定したトークンタイプか確認する
    ///
    /// # Arguments
    ///
    /// * `token_type` - 確認したいトークンタイプ
    ///
    /// # Returns
    ///
    /// * `token_type` - 確認したいトークンタイプ
    fn check(&self, token_type: TokenType) -> bool {
        self.peek().token_type == token_type
    }

    /// 次のトークンが指定したトークンタイプか確認し、一致しない場合はエラーを返す
    ///
    /// # Arguments
    ///
    /// * `token_type` - 確認したいトークンタイプ
    ///
    /// # Returns
    ///
    /// * `Result<(), ParserError>` - 一致した場合はOk、一致しない場合はエラー
    pub fn check_advance(&mut self, token_type: TokenType) -> Result<(), ParserError> {
        if self.check(token_type.clone()) == false {
            return Err(ParserError::MismatchedToken {
                expected: token_type,
                found: self.peek().token_type.clone(),
                line: self.peek().line,
                char_pos: self.peek().char_pos,
            });
        }
        self.advance();
        Ok(())
    }
}
