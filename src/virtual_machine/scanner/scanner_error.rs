use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ScannerError {
    #[error("Unexpected token: {token} at line {line}, position {char_pos}, source code line: {source_code_line}")]
    UnexpectedToken {
        token: String,
        line: usize,
        char_pos: usize,
        source_code_line: String,
    },
    #[error("Invalid Character was Found: {number_literal} at line {line}, position {char_pos}, source code line: {source_code_line}")]
    InvalidCharacterInNumberLiteral {
        number_literal: String,
        line: usize,
        char_pos: usize,
        source_code_line: String,
    },
    #[error("Invalid IntegerLiteral was Found: {number} at line {line}, position {char_pos}, source code line: {source_code_line}")]
    InvalidIntegerLiteralFound {
        number: String,
        line: usize,
        char_pos: usize,
        source_code_line: String,
    },
    #[error("Invalid FloatLiteral was Found: {number} at line {line}, position {char_pos}, source code line: {source_code_line}")]
    InvalidFloatLiteralFound {
        number: String,
        line: usize,
        char_pos: usize,
        source_code_line: String,
    },
    #[error(
        "Unterminated String was Found: {string} at line {line}, position {char_pos}, source code line: {source_code_line}"
    )]
    UnterminatedString {
        string: String,
        line: usize,
        char_pos: usize,
        source_code_line: String,
    },
}
