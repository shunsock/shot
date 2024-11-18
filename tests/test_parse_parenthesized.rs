use shot::virtual_machine::ast::{BinaryOperator, ExpressionNode, LiteralValue};
use shot::virtual_machine::parser::expression_parser::parse_parenthesized::parse_parenthesized;
use shot::virtual_machine::parser::parser_error::ParserError;
use shot::virtual_machine::parser::Parser;
use shot::virtual_machine::token::{token_type::TokenType, Token};

fn create_parser_with_tokens(tokens: Vec<Token>) -> Parser {
    let mut tokens_with_eof = tokens.clone();
    tokens_with_eof.push(Token::new(1, 1, TokenType::Eof)); // EOFを追加
    Parser::new(tokens_with_eof)
}

/// 数値リテラルを含む括弧内の式をパースするテスト
/// test case: `(1)`
#[test]
fn test_parse_valid_parenthesized_literal() {
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

/// 右の括弧がない空の括弧内の式をパースするテスト
/// test case: `(1`
/// 期待されるエラー: UnexpectedTokenType
/// 期待されるエラーメッセージ: "Unexpected token: RightParen"
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

/// 多重の括弧を含む式をパースするテスト
/// test case: `((10))`
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

/// 括弧内の式をパースするテスト
/// test case: `(1 + 1)`
#[test]
fn test_parse_valid_parenthesized_expression() {
    let tokens = vec![
        Token::new(1, 1, TokenType::LeftParen),
        Token::new(1, 2, TokenType::IntegerLiteral(1)),
        Token::new(1, 3, TokenType::Plus),
        Token::new(1, 4, TokenType::IntegerLiteral(1)),
        Token::new(1, 3, TokenType::RightParen),
    ];
    let mut parser = create_parser_with_tokens(tokens);

    let result = parse_parenthesized(&mut parser);
    assert!(result.is_ok());

    let binary_op = match result.unwrap() {
        ExpressionNode::BinaryOperation(binary_op) => binary_op,
        _ => panic!("Expected binary operation inside parentheses"),
    };

    match *binary_op.left {
        ExpressionNode::Literal(literal) => {
            assert_eq!(literal.value, LiteralValue::Integer(1));
        }
        _ => panic!("Expected integer literal inside binary operation"),
    }

    match *binary_op.right {
        ExpressionNode::Literal(literal) => {
            assert_eq!(literal.value, LiteralValue::Integer(1));
        }
        _ => panic!("Expected integer literal inside binary operation"),
    }

    assert_eq!(binary_op.operator, BinaryOperator::Add);
}
