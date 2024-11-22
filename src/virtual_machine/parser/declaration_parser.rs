mod parse_declaration_of_function;
pub mod parse_declaration_of_variable;

use crate::virtual_machine::ast::Statement;
use crate::virtual_machine::parser::Parser;
use crate::virtual_machine::parser::ParserError;
use crate::virtual_machine::token::token_type::TokenType;
use parse_declaration_of_function::parse_declaration_of_function;
use parse_declaration_of_variable::parse_declaration_of_variable;

pub(crate) fn parse_declaration(parser: &mut Parser) -> Result<Statement, ParserError> {
    // 変数宣言と関数宣言をRoute
    // let f # 次の次が `type` なら変数宣言
    // let f # 次の次が `fn` なら関数宣言
    match parser.peek_next_next().token_type {
        TokenType::StringType => Ok(parse_declaration_of_variable(parser)?),
        TokenType::IntType => Ok(parse_declaration_of_variable(parser)?),
        TokenType::FloatType => Ok(parse_declaration_of_variable(parser)?),
        TokenType::VoidType => Ok(parse_declaration_of_variable(parser)?),
        TokenType::Fn => Ok(parse_declaration_of_function(parser)?),
        _ => Err(ParserError::MismatchedToken {
            expected: TokenType::Equal,
            found: parser.peek().token_type.clone(),
            line: parser.peek().line,
            char_pos: parser.peek().char_pos,
        }),
    }
}
