use crate::virtual_machine::token::token_type::TokenType;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
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
    #[error("Unexpected end of input while parsing, expected {expected:?}")]
    UnexpectedEof { expected: TokenType },
    #[error("Not implemented: {feature} at line {line}, position {char_pos}")]
    NotImplementedError {
        feature: String,
        line: usize,
        char_pos: usize,
    },
}
