use crate::virtual_machine::ast::Statement;
use crate::virtual_machine::parser::parser_error::ParserError;
use crate::virtual_machine::parser::Parser;

pub(crate) fn parse_declaration_of_function(parser: &mut Parser) -> Result<Statement, ParserError> {
    Err(ParserError::NotImplementedError {
        feature: "Function declaration".to_string(),
        line: parser.peek().line,
        char_pos: parser.peek().char_pos,
    })
}
