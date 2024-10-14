use crate::virtual_machine::ast::ExpressionNode;
use crate::virtual_machine::parser::expression_parser::parse_binary::parse_binary;
use crate::virtual_machine::parser::parser_error::ParserError;
use crate::virtual_machine::parser::Parser;
use crate::virtual_machine::token::token_type::TokenType;

pub fn parse_parenthesized(parser: &mut Parser) -> Result<ExpressionNode, ParserError> {
    parser.expect(TokenType::LeftParen)?; // 左括弧を消費

    // 中身が空かをチェック
    if parser.check(TokenType::RightParen) {
        return Err(ParserError::UnexpectedTokenType {
            token: parser.peek().token_type.clone(),
            line: parser.peek().line,
            char_pos: parser.peek().char_pos,
        });
    }

    // 括弧内の式をパース (優先順位に基づくパース)
    let expr = parse_binary(parser)?;

    // 右括弧があるかを確認し、なければエラー
    parser
        .expect(TokenType::RightParen)
        .map_err(|_| ParserError::MismatchedToken {
            expected: TokenType::RightParen,
            found: parser.peek().token_type.clone(),
            line: parser.peek().line,
            char_pos: parser.peek().char_pos,
        })?;

    Ok(expr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::virtual_machine::ast::{ExpressionNode, LiteralValue};
    use crate::virtual_machine::parser::Parser;
    use crate::virtual_machine::token::{token_type::TokenType, Token};

    fn create_parser_with_tokens(tokens: Vec<Token>) -> Parser {
        let mut tokens_with_eof = tokens.clone();
        tokens_with_eof.push(Token::new(1, 1, TokenType::Eof)); // EOFを追加
        Parser::new(tokens_with_eof)
    }

    #[test]
    fn test_parse_valid_parenthesized_expression() {
        let tokens = vec![
            Token::new(1, 1, TokenType::LeftParen),
            Token::new(1, 2, TokenType::IntegerLiteral(42)),
            Token::new(1, 3, TokenType::RightParen),
        ];
        let mut parser = create_parser_with_tokens(tokens);

        let result = parse_parenthesized(&mut parser);
        assert!(result.is_ok());

        match result.unwrap() {
            ExpressionNode::Literal(literal) => {
                assert_eq!(literal.value, LiteralValue::Integer(42));
            }
            _ => panic!("Expected integer literal inside parentheses"),
        }
    }

    #[test]
    fn test_parse_empty_parenthesized_expression() {
        let tokens = vec![
            Token::new(1, 1, TokenType::LeftParen),
            Token::new(1, 2, TokenType::RightParen), // 空の括弧
        ];
        let mut parser = create_parser_with_tokens(tokens);

        let result = parse_parenthesized(&mut parser);
        assert!(result.is_err());

        match result {
            Err(ParserError::UnexpectedTokenType { token, .. }) => {
                assert_eq!(token, TokenType::RightParen);
            }
            _ => panic!("Expected UnexpectedTokenType error for empty parentheses"),
        }
    }

    #[test]
    fn test_parse_missing_right_paren() {
        let tokens = vec![
            Token::new(1, 1, TokenType::LeftParen),
            Token::new(1, 2, TokenType::IntegerLiteral(42)),
            // 右括弧がない
        ];
        let mut parser = create_parser_with_tokens(tokens);

        let result = parse_parenthesized(&mut parser);
        assert!(result.is_err());

        match result {
            Err(ParserError::MismatchedToken {
                expected, found, ..
            }) => {
                assert_eq!(expected, TokenType::RightParen);
                assert_eq!(found, TokenType::Eof);
            }
            _ => panic!("Expected MismatchedToken error for missing right parenthesis"),
        }
    }

    #[test]
    fn test_parse_nested_parenthesized_expression() {
        let tokens = vec![
            Token::new(1, 1, TokenType::LeftParen),
            Token::new(1, 2, TokenType::LeftParen),
            Token::new(1, 3, TokenType::IntegerLiteral(10)),
            Token::new(1, 4, TokenType::RightParen),
            Token::new(1, 5, TokenType::RightParen),
        ];
        let mut parser = create_parser_with_tokens(tokens);

        let result = parse_parenthesized(&mut parser);
        assert!(result.is_ok());

        match result.unwrap() {
            ExpressionNode::Literal(literal) => {
                assert_eq!(literal.value, LiteralValue::Integer(10));
            }
            _ => panic!("Expected nested parentheses with integer literal inside"),
        }
    }

    pub fn parse_parenthesized(parser: &mut Parser) -> Result<ExpressionNode, ParserError> {
        parser.expect(TokenType::LeftParen)?; // 左括弧を消費

        // 中身が空かをチェック
        if parser.check(TokenType::RightParen) {
            return Err(ParserError::UnexpectedTokenType {
                token: parser.peek().token_type.clone(),
                line: parser.peek().line,
                char_pos: parser.peek().char_pos,
            });
        }

        // 括弧内の式をパース (優先順位に基づくパース)
        let expr = parse_binary(parser)?;

        // 右括弧があるかを確認し、なければエラー
        parser
            .expect(TokenType::RightParen)
            .map_err(|_| ParserError::MismatchedToken {
                expected: TokenType::RightParen,
                found: parser.peek().token_type.clone(),
                line: parser.peek().line,
                char_pos: parser.peek().char_pos,
            })?;

        Ok(expr)
    }
}
