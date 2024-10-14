use crate::virtual_machine::parser::Parser;
use crate::virtual_machine::parser::ParserError;
use crate::virtual_machine::token::token_type::TokenType;

pub fn expect(parser: &mut Parser, token_type: TokenType) -> Result<(), ParserError> {
    if parser.match_token(token_type.clone()) {
        Ok(())
    } else {
        Err(ParserError::MismatchedToken {
            expected: token_type,
            found: parser.peek().token_type.clone(),
            line: parser.peek().line,
            char_pos: parser.peek().char_pos,
        })
    }
}
