mod parse_declaration_of_function;
mod parse_declaration_of_variable;

use crate::virtual_machine::ast::{Statement, Type};
use crate::virtual_machine::parser::Parser;
use crate::virtual_machine::parser::ParserError;
use crate::virtual_machine::token::token_type::TokenType;
use parse_declaration_of_function::parse_declaration_of_function;
use parse_declaration_of_variable::parse_declaration_of_variable;

pub(crate) fn parse_declaration(parser: &mut Parser) -> Result<Statement, ParserError> {
    // letキーワードを読み飛ばす
    parser.advance();

    // 変数宣言と関数宣言をRoute
    match parser.peek_next().token_type {
        TokenType::Colon => parse_declaration_of_variable(parser),
        TokenType::LeftParen => parse_declaration_of_function(parser),
        _ => Err(ParserError::MismatchedToken {
            expected: TokenType::Equal,
            found: parser.peek().token_type.clone(),
            line: parser.peek().line,
            char_pos: parser.peek().char_pos,
        }),
    }
}
