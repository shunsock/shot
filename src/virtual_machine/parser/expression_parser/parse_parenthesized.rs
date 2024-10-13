use crate::virtual_machine::ast::ExpressionNode;
use crate::virtual_machine::parser::expression_parser::parse_expression;
use crate::virtual_machine::parser::parser_error::ParserError;
use crate::virtual_machine::parser::Parser;
use crate::virtual_machine::token::token_type::TokenType;

pub fn parse_parenthesized(parser: &mut Parser) -> Result<ExpressionNode, ParserError> {
    parser.expect(TokenType::LeftParen)?; // 左括弧を期待
    let expr = parse_expression(parser)?; // 中身の式をパース
    parser.expect(TokenType::RightParen)?; // 右括弧を期待
    Ok(expr)
}
