use crate::virtual_machine::token::token_type::TokenType;
use thiserror::Error;

#[derive(Debug, Clone, Error, PartialEq)]
pub enum ParserError {
    #[error("Unexpected token: {token:?} at line {line}, position {char_pos}")]
    UnexpectedTokenType {
        token: TokenType,
        line: usize,
        char_pos: usize,
    },
    #[error("Expected token of type {expected:?} but found {found:?} at line {line}, position {char_pos}")]
    MismatchedToken {
        expected: TokenType,
        found: TokenType,
        line: usize,
        char_pos: usize,
    },
    #[error("could not found {found:?} type in Shot at line {line}, position {char_pos}")]
    TypeNotFound {
        found: TokenType,
        line: usize,
        char_pos: usize,
    },
    #[error("Unexpected Eof Found (expected {expected:?}) at line {line}, position {char_pos}")]
    UnexpectedEof {
        expected: TokenType,
        line: usize,
        char_pos: usize,
    },
}
