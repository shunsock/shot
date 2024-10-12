pub mod token_type;

use crate::virtual_machine::token::token_type::TokenType;

#[derive(Clone, PartialEq)]
pub(crate) struct Token {
    line: usize,
    char_pos: usize,
    pub token_type: TokenType,
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
