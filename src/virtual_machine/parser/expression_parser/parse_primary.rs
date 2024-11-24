use crate::virtual_machine::ast::{ExpressionNode, LiteralNode, LiteralValue};
use crate::virtual_machine::parser::expression_parser::parse_call_of_variable_and_function::parse_identifier_or_call;
use crate::virtual_machine::parser::expression_parser::parse_parenthesized::parse_parenthesized;
use crate::virtual_machine::parser::expression_parser::parse_type_cast::parse_type_cast;
use crate::virtual_machine::parser::parser_error::ParserError;
use crate::virtual_machine::parser::Parser;
use crate::virtual_machine::token::token_type::TokenType;

/// Literalをparseする関数
///
/// # Arguments
/// * `parser` - パーサー
///
/// # Returns
/// * `ExpressionNode` - リテラルのASTノード
/// * `ParserError` - パーサーエラー
pub fn parse_primary(parser: &mut Parser) -> Result<ExpressionNode, ParserError> {
    let expr = match parser.peek().token_type.clone() {
        // 整数リテラル
        TokenType::IntegerLiteral(value) => {
            parser.advance();
            ExpressionNode::Literal(Box::new(LiteralNode {
                value: LiteralValue::Integer(value),
            }))
        }
        // 浮動小数点リテラル
        TokenType::FloatLiteral(value) => {
            parser.advance();
            ExpressionNode::Literal(Box::new(LiteralNode {
                value: LiteralValue::Float(value),
            }))
        }
        // 文字列リテラル
        TokenType::StringLiteral(ref value) => {
            parser.advance();
            ExpressionNode::Literal(Box::new(LiteralNode {
                value: LiteralValue::String(value.clone()),
            }))
        }
        // Noneリテラル
        TokenType::NoneLiteral => {
            parser.advance();
            ExpressionNode::Literal(Box::new(LiteralNode {
                value: LiteralValue::None,
            }))
        }
        // 識別子や関数呼び出し
        TokenType::Identifier(_) => {
            // 識別子または関数呼び出しのパース
            parse_identifier_or_call(parser)?
        }
        // 括弧付きの式
        TokenType::LeftParen => {
            // 括弧付き式のパース
            parse_parenthesized(parser)?
        }
        // 他のリテラルが必要な場合に追加
        _ => {
            return Err(ParserError::UnexpectedTokenType {
                token: parser.peek().token_type.clone(),
                line: parser.peek().line,
                char_pos: parser.peek().char_pos,
            })
        }
    };

    match parser.peek().token_type.clone() {
        TokenType::As => parse_type_cast(parser, expr),
        _ => Ok(expr),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::virtual_machine::ast::{ExpressionNode, LiteralValue};
    use crate::virtual_machine::parser::Parser;
    use crate::virtual_machine::token::{token_type::TokenType, Token};

    fn create_parser_with_token(token: Token) -> Parser {
        Parser::new(vec![token, Token::new(1, 1, TokenType::Eof)])
    }

    /// 行と位置を指定したトークンを持つパーサーを生成する
    fn create_parser_with_tokens() -> Parser {
        let tokens = vec![
            Token::new(1, 2, TokenType::IntegerLiteral(42)),
            Token::new(2, 5, TokenType::FloatLiteral(3.14)),
            Token::new(3, 10, TokenType::StringLiteral("test".to_string())),
            Token::new(4, 1, TokenType::NoneLiteral),
        ];
        Parser::new(tokens)
    }

    /// 整数リテラルをパース可能か確認するテスト
    #[test]
    fn test_parse_integer_literal() {
        let token = Token::new(1, 1, TokenType::IntegerLiteral(42));
        let mut parser = create_parser_with_token(token);
        let result = parse_primary(&mut parser);

        assert!(result.is_ok());
        if let ExpressionNode::Literal(literal) = result.unwrap() {
            assert_eq!(literal.value, LiteralValue::Integer(42));
        } else {
            panic!("Expected integer literal");
        }
    }

    /// 浮動小数点リテラルをパース可能か確認するテスト
    #[test]
    fn test_parse_float_literal() {
        let token = Token::new(1, 1, TokenType::FloatLiteral(3.14));
        let mut parser = create_parser_with_token(token);
        let result = parse_primary(&mut parser);

        assert!(result.is_ok());
        if let ExpressionNode::Literal(literal) = result.unwrap() {
            assert_eq!(literal.value, LiteralValue::Float(3.14));
        } else {
            panic!("Expected float literal");
        }
    }

    /// 文字列リテラルをパース可能か確認するテスト
    #[test]
    fn test_parse_string_literal() {
        let token = Token::new(1, 1, TokenType::StringLiteral("Hello".to_string()));
        let mut parser = create_parser_with_token(token);
        let result = parse_primary(&mut parser);

        assert!(result.is_ok());
        if let ExpressionNode::Literal(literal) = result.unwrap() {
            assert_eq!(literal.value, LiteralValue::String("Hello".to_string()));
        } else {
            panic!("Expected string literal");
        }
    }

    /// Noneリテラルをパース可能か確認するテスト
    #[test]
    fn test_parse_none_literal() {
        let token = Token::new(1, 1, TokenType::NoneLiteral);
        let mut parser = create_parser_with_token(token);
        let result = parse_primary(&mut parser);

        assert!(result.is_ok());
        if let ExpressionNode::Literal(literal) = result.unwrap() {
            assert_eq!(literal.value, LiteralValue::None);
        } else {
            panic!("Expected None literal");
        }
    }

    /// 予期しないトークンが出現した際にエラーを返すか確認するテスト
    #[test]
    fn test_unexpected_token_type() {
        let token = Token::new(1, 1, TokenType::Plus); // 予期しないトークン
        let mut parser = create_parser_with_token(token);
        let result = parse_primary(&mut parser);

        assert!(result.is_err());
        if let Err(ParserError::UnexpectedTokenType {
            token,
            line,
            char_pos,
        }) = result
        {
            assert_eq!(token, TokenType::Plus);
            assert_eq!(line, 1);
            assert_eq!(char_pos, 1);
        } else {
            panic!("Expected UnexpectedTokenType error");
        }
    }
}
