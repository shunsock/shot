pub mod token_type;

use crate::virtual_machine::token::token_type::TokenType;

#[derive(Clone, PartialEq, Debug)]
pub struct Token {
    pub(crate) line: usize,
    pub(crate) char_pos: usize,
    pub(crate) token_type: TokenType,
}

impl Token {
    pub fn new(line: usize, char_pos: usize, token_type: TokenType) -> Self {
        Token {
            line,
            char_pos,
            token_type,
        }
    }
}
