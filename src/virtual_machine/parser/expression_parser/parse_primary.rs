use crate::virtual_machine::ast::{ExpressionNode, LiteralNode, LiteralValue};
use crate::virtual_machine::parser::expression_parser::parse_call_of_variable_and_function::parse_identifier_or_call;
use crate::virtual_machine::parser::expression_parser::parse_parenthesized::parse_parenthesized;
use crate::virtual_machine::parser::expression_parser::parse_type_cast::parse_type_cast;
use crate::virtual_machine::parser::parser_error::ParserError;
use crate::virtual_machine::parser::Parser;
use crate::virtual_machine::token::token_type::TokenType;

/// Primary表現をparseする関数
///
/// # Returns
/// - `Result<ExpressionNode, ParserError>` - パース結果
///  - `ExpressionNode` - 式ノード
///  - `ParserError` - パースエラー
///
/// # Syntax
/// `integer_literal | float_literal | string_literal | none_literal | variable_call | function_call | parenthesized`
///
/// # Example
/// - `42`
/// - `3.14`
/// - `"Hello"`
/// - `None`
/// - `a`
/// - `add(1, 2)`
/// - `(1 + 2)`
pub fn parse_primary(parser: &mut Parser) -> Result<ExpressionNode, ParserError> {
    let expr = match parser.peek().token_type.clone() {
        TokenType::LeftParen => {
            // 括弧内の式のパース
            parse_parenthesized(parser)?
        }
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
    use crate::virtual_machine::parser::core::create_parser_with_tokens;
    use crate::virtual_machine::parser::Parser;
    use crate::virtual_machine::token::{token_type::TokenType, Token};

    /// 整数リテラルをパース可能か確認するテスト
    /// 42;
    #[test]
    fn test_parse_integer_literal() {
        // 生成されるAST Node
        let expected = Box::new(LiteralNode {
            value: LiteralValue::Integer(42),
        });

        // テストする関数の入力である、Token列, Parserの生成
        // 42;
        let tokens: Vec<Token> = vec![Token::new(1, 1, TokenType::IntegerLiteral(42))];
        let mut parser: Parser = create_parser_with_tokens(tokens);

        // テスト対象の関数の実行(エラーが出ないことを確認)
        let result: Result<ExpressionNode, ParserError> = parse_primary(&mut parser);
        assert!(result.is_ok());

        // テスト対象の関数の実行結果が期待値と一致することを確認
        let actual: Box<LiteralNode> = match result.unwrap() {
            ExpressionNode::Literal(literal) => literal,
            _ => panic!("Expected literal node"),
        };
        assert_eq!(actual, expected);
    }

    /// 浮動小数点リテラルをパース可能か確認するテスト
    /// 3.14;
    #[test]
    fn test_parse_float_literal() {
        // 生成されるAST Node
        let expected = Box::new(LiteralNode {
            value: LiteralValue::Float(3.14),
        });

        // テストする関数の入力である、Token列, Parserの生成
        // 3.14;
        let tokens: Vec<Token> = vec![Token::new(1, 1, TokenType::FloatLiteral(3.14))];
        let mut parser: Parser = create_parser_with_tokens(tokens);

        // テスト対象の関数の実行(エラーが出ないことを確認)
        let result: Result<ExpressionNode, ParserError> = parse_primary(&mut parser);
        assert!(result.is_ok());

        // テスト対象の関数の実行結果が期待値と一致することを確認
        let actual: Box<LiteralNode> = match result.unwrap() {
            ExpressionNode::Literal(literal) => literal,
            _ => panic!("Expected literal node"),
        };
        assert_eq!(actual, expected);
    }

    /// 文字列リテラルをパース可能か確認するテスト
    /// "Hello";
    #[test]
    fn test_parse_string_literal() {
        // 生成されるAST Node
        let expected = Box::new(LiteralNode {
            value: LiteralValue::String("Hello".to_string()),
        });

        // テストする関数の入力である、Token列, Parserの生成
        // "Hello";
        let tokens: Vec<Token> = vec![Token::new(
            1,
            1,
            TokenType::StringLiteral("Hello".to_string()),
        )];
        let mut parser: Parser = create_parser_with_tokens(tokens);

        // テスト対象の関数の実行(エラーが出ないことを確認)
        let result: Result<ExpressionNode, ParserError> = parse_primary(&mut parser);
        assert!(result.is_ok());

        // テスト対象の関数の実行結果が期待値と一致することを確認
        let actual: Box<LiteralNode> = match result.unwrap() {
            ExpressionNode::Literal(literal) => literal,
            _ => panic!("Expected literal node"),
        };
        assert_eq!(actual, expected);
    }

    /// Noneリテラルをパース可能か確認するテスト
    /// none;
    #[test]
    fn test_parse_none_literal() {
        // 生成されるAST Node
        let expected = Box::new(LiteralNode {
            value: LiteralValue::None,
        });

        // テストする関数の入力である、Token列, Parserの生成
        // none;
        let tokens: Vec<Token> = vec![Token::new(1, 1, TokenType::NoneLiteral)];
        let mut parser: Parser = create_parser_with_tokens(tokens);

        // テスト対象の関数の実行(エラーが出ないことを確認)
        let result: Result<ExpressionNode, ParserError> = parse_primary(&mut parser);
        assert!(result.is_ok());

        // テスト対象の関数の実行結果が期待値と一致することを確認
        let actual: Box<LiteralNode> = match result.unwrap() {
            ExpressionNode::Literal(literal) => literal,
            _ => panic!("Expected literal node"),
        };
        assert_eq!(actual, expected);
    }

    /// 予期しないトークンが出現した際にエラーを返すか確認するテスト
    /// +;
    #[test]
    fn test_unexpected_token_type() {
        // 想定されるエラー
        let expected = ParserError::UnexpectedTokenType {
            token: TokenType::Plus,
            line: 1,
            char_pos: 1,
        };

        // テストする関数の入力である、Token列, Parserの生成
        let tokens: Vec<Token> = vec![Token::new(1, 1, TokenType::Plus)]; // 予期しないトークン
        let mut parser: Parser = create_parser_with_tokens(tokens);

        // テスト対象の関数の実行(エラーが出ることを確認)
        let result: Result<ExpressionNode, ParserError> = parse_primary(&mut parser);
        assert!(result.is_err());

        // テスト対象の関数の実行結果が期待値と一致することを確認
        let actual: ParserError = match result {
            Err(err) => err,
            _ => panic!("Expected error, but got Ok"),
        };
        assert_eq!(actual, expected);
    }
}
