use crate::virtual_machine::ast::{BinaryOperationNode, BinaryOperator, ExpressionNode};
use crate::virtual_machine::parser::expression_parser::parse_primary::parse_primary;
use crate::virtual_machine::parser::parser_error::ParserError;
use crate::virtual_machine::parser::Parser;
use crate::virtual_machine::token::token_type::TokenType;

/// 二項演算の解析
///
/// # Returns
/// - `Result<ExpressionNode, ParserError>` - パース結果
///  - `ExpressionNode` - 式ノード
///  - `ParserError` - パースエラー
///
/// # Syntax
/// 二項演算は左辺と右辺の式ノードに対して演算子を適用する。
///
/// - `left operator right`
///
/// # Examples
/// - `1 + 2`
/// - `1 - 2`
/// - `(1 + 2) * 3`
pub fn parse_binary(parser: &mut Parser) -> Result<ExpressionNode, ParserError> {
    parse_addition_subtraction(parser)
}

/// 加算・減算の解析（優先順位低）
///
/// # Returns
/// - `Result<ExpressionNode, ParserError>` - パース結果
///   - `ExpressionNode` - 式ノード
///   - `ParserError` - パースエラー
///
/// # Syntax
/// 加算は以下のように表現される。
/// - `1 + 2`
///
/// # Notes
/// BinaryOperationNode には左辺と右辺の式ノードが格納される。
///
/// # Details
/// primary の処理はこの関数では行わない。
/// `parse_multiplication_division` を経由して `parse_primary` で行う。
///
/// この関数のwhileループが開始する時点で以下の要件を満たしている
/// - 左辺の式ノードが存在する
/// - 演算子が存在する
/// - TokenType::Asterisk | TokenType::Slash は parse_multiplication_division で処理済み
fn parse_addition_subtraction(parser: &mut Parser) -> Result<ExpressionNode, ParserError> {
    // 優先度の高い乗算・除算を先に処理
    let mut node: ExpressionNode = parse_multiplication_division(parser)?;

    while let Some(_token) = match parser.peek().token_type {
        TokenType::Plus | TokenType::Minus => Some(parser.peek().token_type.clone()),
        _ => None,
    } {
        let operator: BinaryOperator = match parser.advance().token_type {
            TokenType::Plus => BinaryOperator::Add,
            TokenType::Minus => BinaryOperator::Subtract,
            _ => unreachable!(),
        };

        let right: ExpressionNode = parse_multiplication_division(parser)?;
        node = ExpressionNode::BinaryOperation(Box::new(BinaryOperationNode {
            left: Box::new(node),
            operator,
            right: Box::new(right),
        }));
    }

    Ok(node)
}

/// 乗算・除算の解析（優先順位高）
///
/// # Returns
/// - `Result<ExpressionNode, ParserError>` - パース結果
///   - `ExpressionNode` - 式ノード
///   - `ParserError` - パースエラー
///
/// # Syntax
/// 乗算は以下のように表現される。
/// - `1 * 2`
///
/// # Notes
/// BinaryOperationNode には左辺と右辺の式ノードが格納される。
///
/// # Details
/// 最上位の優先度をもつので、left には primary が入る。
/// primary の処理はこの関数では行わない。`parse_primary` で行う。
/// parenthesis の処理はこの関数では行わない。`parse_primary` で行う。
fn parse_multiplication_division(parser: &mut Parser) -> Result<ExpressionNode, ParserError> {
    let mut node: ExpressionNode = parse_primary(parser)?;

    while let Some(_token) = match parser.peek().token_type {
        TokenType::Asterisk | TokenType::Slash => Some(parser.peek().token_type.clone()),
        _ => None,
    } {
        let operator: BinaryOperator = match parser.advance().token_type {
            TokenType::Asterisk => BinaryOperator::Multiply,
            TokenType::Slash => BinaryOperator::Divide,
            _ => unreachable!(),
        };

        let right: ExpressionNode = parse_primary(parser)?;
        node = ExpressionNode::BinaryOperation(Box::new(BinaryOperationNode {
            left: Box::new(node),
            operator,
            right: Box::new(right),
        }));
    }

    Ok(node)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::virtual_machine::ast::{ExpressionNode, LiteralValue};
    use crate::virtual_machine::parser::core::create_parser_with_tokens;
    use crate::virtual_machine::token::{token_type::TokenType, Token};

    /// 単純な加算式をパースするテスト
    /// 1 + 2
    #[test]
    fn test_parse_simple_addition() {
        let tokens = vec![
            Token::new(1, 1, TokenType::IntegerLiteral(1)),
            Token::new(1, 2, TokenType::Plus),
            Token::new(1, 3, TokenType::IntegerLiteral(2)),
        ];
        let mut parser = create_parser_with_tokens(tokens);

        let result = parse_binary(&mut parser);
        assert!(result.is_ok());

        let binary_op = match result.unwrap() {
            ExpressionNode::BinaryOperation(binary_op) => binary_op,
            _ => panic!("Expected a binary operation"),
        };
        assert_eq!(binary_op.operator, BinaryOperator::Add);

        let left_literal = match *binary_op.left {
            ExpressionNode::Literal(ref literal) => literal,
            _ => panic!("Expected left side to be a literal"),
        };
        assert_eq!(left_literal.value, LiteralValue::Integer(1));

        let right_literal = match *binary_op.right {
            ExpressionNode::Literal(ref literal) => literal,
            _ => panic!("Expected right side to be a literal"),
        };
        assert_eq!(right_literal.value, LiteralValue::Integer(2));
    }

    /// 単純な割り算をパースするテスト
    /// 1 / 2
    #[test]
    fn test_parse_simple_division() {
        let tokens = vec![
            Token::new(1, 1, TokenType::IntegerLiteral(1)),
            Token::new(1, 2, TokenType::Slash),
            Token::new(1, 3, TokenType::IntegerLiteral(2)),
        ];
        let mut parser = create_parser_with_tokens(tokens);

        let result = parse_binary(&mut parser);
        assert!(result.is_ok());

        let binary_op = match result.unwrap() {
            ExpressionNode::BinaryOperation(binary_op) => binary_op,
            _ => panic!("Expected a binary operation"),
        };
        assert_eq!(binary_op.operator, BinaryOperator::Divide);

        let left_literal = match *binary_op.left {
            ExpressionNode::Literal(ref literal) => literal,
            _ => panic!("Expected left side to be a literal"),
        };
        assert_eq!(left_literal.value, LiteralValue::Integer(1));

        let right_literal = match *binary_op.right {
            ExpressionNode::Literal(ref literal) => literal,
            _ => panic!("Expected right side to be a literal"),
        };
        assert_eq!(right_literal.value, LiteralValue::Integer(2));
    }

    /// 乗算が含まれた式をパースするテスト
    /// 1 + 2 * 3
    #[test]
    fn test_parse_multiplication_with_precedence() {
        let tokens = vec![
            Token::new(1, 1, TokenType::IntegerLiteral(1)),
            Token::new(1, 2, TokenType::Plus),
            Token::new(1, 3, TokenType::IntegerLiteral(2)),
            Token::new(1, 4, TokenType::Asterisk),
            Token::new(1, 5, TokenType::IntegerLiteral(3)),
        ];
        let mut parser = create_parser_with_tokens(tokens);

        let result = parse_binary(&mut parser);
        assert!(result.is_ok());

        let binary_op = match result.unwrap() {
            ExpressionNode::BinaryOperation(binary_op) => binary_op,
            _ => panic!("Expected a binary operation"),
        };
        assert_eq!(binary_op.operator, BinaryOperator::Add);

        let left_literal = match *binary_op.left {
            ExpressionNode::Literal(ref literal) => literal,
            _ => panic!("Expected left side to be a literal"),
        };
        assert_eq!(left_literal.value, LiteralValue::Integer(1));

        let mul_op = match *binary_op.right {
            ExpressionNode::BinaryOperation(mul_op) => mul_op,
            _ => panic!("Expected right side to be a multiplication operation"),
        };
        assert_eq!(mul_op.operator, BinaryOperator::Multiply);

        let left_mul_literal = match *mul_op.left {
            ExpressionNode::Literal(ref literal) => literal,
            _ => panic!("Expected left side of multiplication to be a literal"),
        };
        assert_eq!(left_mul_literal.value, LiteralValue::Integer(2));

        let right_mul_literal = match *mul_op.right {
            ExpressionNode::Literal(ref literal) => literal,
            _ => panic!("Expected right side of multiplication to be a literal"),
        };
        assert_eq!(right_mul_literal.value, LiteralValue::Integer(3));
    }

    /// 括弧が含まれた式をパースするテスト
    /// (1 + 2) * 3
    #[test]
    fn test_parse_with_parentheses() {
        let tokens = vec![
            Token::new(1, 1, TokenType::LeftParen),
            Token::new(1, 2, TokenType::IntegerLiteral(1)),
            Token::new(1, 3, TokenType::Plus),
            Token::new(1, 4, TokenType::IntegerLiteral(2)),
            Token::new(1, 5, TokenType::RightParen),
            Token::new(1, 6, TokenType::Asterisk),
            Token::new(1, 7, TokenType::IntegerLiteral(3)),
        ];
        let mut parser = create_parser_with_tokens(tokens);

        let result = parse_binary(&mut parser);
        assert!(result.is_ok());

        let binary_op = match result.unwrap() {
            ExpressionNode::BinaryOperation(binary_op) => binary_op,
            _ => panic!("Expected a multiplication operation"),
        };
        assert_eq!(binary_op.operator, BinaryOperator::Multiply);

        let add_op = match *binary_op.left {
            ExpressionNode::BinaryOperation(add_op) => add_op,
            _ => panic!("Expected left side to be an addition operation"),
        };
        assert_eq!(add_op.operator, BinaryOperator::Add);

        let left_add_literal = match *add_op.left {
            ExpressionNode::Literal(ref literal) => literal,
            _ => panic!("Expected left side of addition to be a literal"),
        };
        assert_eq!(left_add_literal.value, LiteralValue::Integer(1));

        let right_add_literal = match *add_op.right {
            ExpressionNode::Literal(ref literal) => literal,
            _ => panic!("Expected right side of addition to be a literal"),
        };
        assert_eq!(right_add_literal.value, LiteralValue::Integer(2));

        let right_mul_literal = match *binary_op.right {
            ExpressionNode::Literal(ref literal) => literal,
            _ => panic!("Expected right side to be a literal"),
        };
        assert_eq!(right_mul_literal.value, LiteralValue::Integer(3));
    }

    /// 複数の加算が含まれた式をパースするテスト
    /// 1 + 2 - 3
    #[test]
    fn test_parse_multiple_additions() {
        let tokens = vec![
            Token::new(1, 1, TokenType::IntegerLiteral(1)),
            Token::new(1, 2, TokenType::Plus),
            Token::new(1, 3, TokenType::IntegerLiteral(2)),
            Token::new(1, 4, TokenType::Minus),
            Token::new(1, 5, TokenType::IntegerLiteral(3)),
        ];
        let mut parser = create_parser_with_tokens(tokens);

        let result = parse_binary(&mut parser);
        assert!(result.is_ok());

        let binary_op = match result.unwrap() {
            ExpressionNode::BinaryOperation(binary_op) => binary_op,
            _ => panic!("Expected a binary operation"),
        };
        assert_eq!(binary_op.operator, BinaryOperator::Subtract);

        let left_add_op = match *binary_op.left {
            ExpressionNode::BinaryOperation(add_op) => add_op,
            _ => panic!("Expected left side to be an addition operation"),
        };
        assert_eq!(left_add_op.operator, BinaryOperator::Add);

        let left_add_literal = match *left_add_op.left {
            ExpressionNode::Literal(ref literal) => literal,
            _ => panic!("Expected left side of addition to be a literal"),
        };
        assert_eq!(left_add_literal.value, LiteralValue::Integer(1));

        let right_add_literal = match *left_add_op.right {
            ExpressionNode::Literal(ref literal) => literal,
            _ => panic!("Expected right side of addition to be a literal"),
        };
        assert_eq!(right_add_literal.value, LiteralValue::Integer(2));

        let right_sub_literal = match *binary_op.right {
            ExpressionNode::Literal(ref literal) => literal,
            _ => panic!("Expected right side of subtraction to be a literal"),
        };
        assert_eq!(right_sub_literal.value, LiteralValue::Integer(3));
    }
}
