use crate::virtual_machine::ast::{ExpressionNode, LiteralNode, LiteralValue};
use crate::virtual_machine::parser::expression_parser::parse_expression;
use crate::virtual_machine::parser::parser_error::ParserError;
use crate::virtual_machine::parser::Parser;
use crate::virtual_machine::token::token_type::TokenType;

/// 括弧式をパース
///
/// # Returns
/// - `Result<ExpressionNode, ParserError>` - パース結果
///   - `ExpressionNode` - 式ノード
///   - `ParserError` - パースエラー
///
/// ## Syntax
///
/// 括弧式は 式を `(` と `)` で囲んだ式です。
/// - `(` expression `)`
///
/// () や (()) のような括弧内の式の場合、LiteralNode::NoneなるExpressionNodeを返します。
/// - `() # Void型のnoneリテラルを返す`
///
/// ## Example
/// - `()`
/// - `(1 + 2)`
/// - `(a)`
/// - `(add(1, 2) as int -> float)`
pub fn parse_parenthesized(parser: &mut Parser) -> Result<ExpressionNode, ParserError> {
    // 左括弧がなければエラー、あれば次のトークンに進む
    parser.check_advance(TokenType::LeftParen)?;

    // 中身が空かをチェック
    if parser.check(TokenType::RightParen) {
        return Ok(ExpressionNode::Literal(Box::new(LiteralNode {
            value: LiteralValue::None,
        })));
    }

    // 括弧内の式をパース (優先順位に基づくパース)
    let expr: ExpressionNode = parse_expression(parser)?;

    // 右括弧があるかを確認し、なければエラー
    parser
        .check_advance(TokenType::RightParen)
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
    use crate::virtual_machine::parser::core::create_parser_with_tokens;
    use crate::virtual_machine::parser::Parser;
    use crate::virtual_machine::token::{token_type::TokenType, Token};

    /// 空の括弧内の式をパースするテスト
    /// test case: `()`
    #[test]
    fn test_parse_empty_parenthesized_expression() {
        let tokens: Vec<Token> = vec![
            Token::new(1, 1, TokenType::LeftParen),
            Token::new(1, 2, TokenType::RightParen), // 空の括弧
        ];
        let mut parser: Parser = create_parser_with_tokens(tokens);

        let result: Result<ExpressionNode, ParserError> = parse_parenthesized(&mut parser);
        assert!(result.is_ok());

        let expr: ExpressionNode = result.unwrap();
        assert_eq!(
            expr,
            ExpressionNode::Literal(Box::new(LiteralNode {
                value: LiteralValue::None,
            }))
        );
    }

    /// 多重の括弧を含む式をパースするテスト
    /// test case: `(())`
    #[test]
    fn test_parse_nested_parenthesized_expression() {
        let tokens: Vec<Token> = vec![
            Token::new(1, 1, TokenType::LeftParen),
            Token::new(1, 2, TokenType::LeftParen),
            Token::new(1, 4, TokenType::RightParen),
            Token::new(1, 5, TokenType::RightParen),
        ];
        let mut parser: Parser = create_parser_with_tokens(tokens);

        let result: Result<ExpressionNode, ParserError> = parse_parenthesized(&mut parser);
        assert!(result.is_ok());

        let expr: ExpressionNode = result.unwrap();
        assert_eq!(
            expr,
            ExpressionNode::Literal(Box::new(LiteralNode {
                value: LiteralValue::None,
            }))
        );
    }
}
