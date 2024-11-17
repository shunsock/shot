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

    pub fn parse(&mut self) -> Result<AST, ParserError> {
        let mut ast: AST = AST::new();

        while !self.check(TokenType::Eof) {
            let line: usize = self.peek().line;
            let statement: Statement = parse_statement(self)?;

            ast.push_statement(line, statement)
        }

        Ok(ast)
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
    fn check_advance(&mut self, token_type: TokenType) -> Result<(), ParserError> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::virtual_machine::token::token_type::TokenType;
    use crate::virtual_machine::token::Token;

    /// テスト用のパーサーを生成するヘルパー関数。
    ///
    /// 渡されたトークンリストの末尾に自動的に `Eof` トークンを追加し、
    /// そのトークンリストを元に `Parser` を初期化します。
    ///
    /// # 引数
    /// * `tokens` - 解析対象のトークンのベクター
    ///
    /// # 戻り値
    /// * `Parser` - 与えられたトークンに基づいて初期化されたパーサー
    fn create_parser_with_tokens(mut tokens: Vec<Token>) -> Parser {
        tokens.push(Token::new(0, 0, TokenType::Eof)); // 自動的にEOFトークンを追加
        Parser::new(tokens)
    }

    /// `peek` 関数のテスト。
    ///
    /// 現在のトークンを進めずに取得できることを確認します。
    #[test]
    fn test_peek() {
        let tokens = vec![Token::new(1, 1, TokenType::IntegerLiteral(42))];
        let parser = create_parser_with_tokens(tokens);

        // `peek` が現在のトークンを返すか確認
        let token: &Token = parser.peek();
        assert_eq!(token.token_type, TokenType::IntegerLiteral(42));
    }

    /// `peek_next` 関数のテスト。
    ///
    /// 次のトークンを進めずに取得できることを確認します。
    #[test]
    fn test_peek_next() {
        let tokens = vec![
            Token::new(1, 1, TokenType::IntegerLiteral(42)),
            Token::new(1, 2, TokenType::Plus),
        ];
        let parser = create_parser_with_tokens(tokens);

        // `peek_next` が次のトークンを返すか確認
        let token: &Token = parser.peek_next();
        assert_eq!(token.token_type, TokenType::Plus);
    }

    /// `advance` 関数のテスト。
    ///
    /// 現在のトークンを進め、次のトークンを取得できるか確認します。
    #[test]
    fn test_advance() {
        let tokens = vec![
            Token::new(1, 1, TokenType::IntegerLiteral(42)),
            Token::new(1, 2, TokenType::Plus),
        ];
        let mut parser = create_parser_with_tokens(tokens);

        // `advance` が現在のトークンを進めるか確認
        let token: &Token = parser.advance();
        assert_eq!(token.token_type, TokenType::IntegerLiteral(42));

        // 次のトークンが `Plus` か確認
        let token = parser.peek();
        assert_eq!(token.token_type, TokenType::Plus);
    }

    /// `check` 関数のテスト。
    ///
    /// 指定されたトークンタイプが次のトークンと一致するか確認します。
    #[test]
    fn test_check() {
        let tokens = vec![
            Token::new(1, 1, TokenType::IntegerLiteral(42)),
            Token::new(1, 2, TokenType::Plus),
        ];
        let parser = create_parser_with_tokens(tokens);

        // `check` が正しく動作するか確認
        assert!(parser.check(TokenType::IntegerLiteral(42)));
        assert!(!parser.check(TokenType::Plus));
    }

    /// `check_advance` 関数のテスト。
    ///
    /// 次のトークンが期待されるトークンタイプと一致する場合、トークンを進めることを確認し、
    /// 一致しない場合はエラーを返すことを確認します。
    #[test]
    fn test_check_advance() {
        let tokens = vec![
            Token::new(1, 1, TokenType::IntegerLiteral(42)),
            Token::new(1, 2, TokenType::Plus),
        ];
        let mut parser = create_parser_with_tokens(tokens);

        // `check_advance` が一致した場合、トークンを進めるか確認
        assert!(parser.check_advance(TokenType::IntegerLiteral(42)).is_ok());
        assert_eq!(parser.peek().token_type, TokenType::Plus);

        // `check_advance` が一致しない場合、エラーを返すか確認
        let result: Result<(), ParserError> = parser.check_advance(TokenType::Minus);
        assert!(result.is_err());

        // エラーチェック
        match result {
            Err(ParserError::MismatchedToken {
                expected, found, ..
            }) => {
                assert_eq!(expected, TokenType::Minus);
                assert_eq!(found, TokenType::Plus);
            }
            _ => panic!("Expected MismatchedToken error"),
        }
    }
}
