pub mod parse_binary;
pub mod parse_call_of_variable_and_function;
pub mod parse_parenthesized;
pub mod parse_primary;

use crate::virtual_machine::ast::ExpressionNode;
use crate::virtual_machine::parser::parser_error::ParserError;
use crate::virtual_machine::parser::Parser;
use crate::virtual_machine::token::token_type::TokenType;
use parse_binary::parse_binary;
use parse_call_of_variable_and_function::parse_identifier_or_call;
use parse_parenthesized::parse_parenthesized;

pub fn parse_expression(parser: &mut Parser) -> Result<ExpressionNode, ParserError> {
    if parser.check(TokenType::LeftParen) {
        parse_parenthesized(parser)
    } else if let TokenType::Identifier(_) = parser.peek().token_type {
        parse_identifier_or_call(parser)
    } else {
        parse_binary(parser)
    }
}
