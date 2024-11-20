use crate::virtual_machine::ast::Statement;
use crate::virtual_machine::parser::parser_error::ParserError;
use crate::virtual_machine::parser::Parser;

/// ## 関数宣言をパースする関数
///
/// ## Example
/// let f: fn = () => { *Statements* };
/// let tokenは上流の `declaration_parser.rs` の `parse_declaration` で消費されている
/// semicolon tokenは上流の `statement_parser.rs` の `parse_statement` で消費される
/// この関数では、 f: fn = () => { *Statements* } の部分をパースする
pub(crate) fn parse_declaration_of_function(parser: &mut Parser) -> Result<Statement, ParserError> {
    Err(ParserError::NotImplementedError {
        feature: "Function declaration".to_string(),
        line: parser.peek().line,
        char_pos: parser.peek().char_pos,
    })
}
