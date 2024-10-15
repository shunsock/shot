use crate::virtual_machine::ast::Type;
use crate::virtual_machine::parser::Parser;
use crate::virtual_machine::parser::ParserError;
use crate::virtual_machine::token::token_type::TokenType;

pub fn expect(parser: &mut Parser, token_type: TokenType) -> Result<(), ParserError> {
    if parser.check(token_type.clone()) {
        parser.advance();
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

pub fn type_token_to_type(token_type: TokenType) -> Result<Type, ParserError> {
    match token_type {
        TokenType::IntType => Ok(Type::Integer),
        TokenType::FloatType => Ok(Type::Float),
        TokenType::StringType => Ok(Type::String),
        TokenType::VoidType => Ok(Type::Void),
        TokenType::Fn => Ok(Type::Function),
        _ => Err(ParserError::TypeNotFound {
            found: token_type,
            line: 0,
            char_pos: 0,
        }),
    }
}
