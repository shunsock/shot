pub mod parse_binary;
pub mod parse_call_of_variable_and_function;
pub mod parse_parenthesized;
pub mod parse_primary;
mod parse_type_cast;

use crate::virtual_machine::ast::ExpressionNode;
use crate::virtual_machine::parser::expression_parser::parse_type_cast::parse_type_cast;
use crate::virtual_machine::parser::parser_error::ParserError;
use crate::virtual_machine::parser::Parser;
use crate::virtual_machine::token::token_type::TokenType;
use parse_binary::parse_binary;
use parse_parenthesized::parse_parenthesized;

pub fn parse_expression(parser: &mut Parser) -> Result<ExpressionNode, ParserError> {
    let expr: ExpressionNode = if parser.check(TokenType::LeftParen) {
        parse_parenthesized(parser)?
    } else {
        parse_binary(parser)?
    };

    match parser.peek().token_type.clone() {
        TokenType::As => parse_type_cast(parser, expr),
        _ => Ok(expr),
    }
}
