use crate::virtual_machine::ast::Type;
use crate::virtual_machine::parser::ParserError;
use crate::virtual_machine::token::token_type::TokenType;

/// 入力されたトークンをShot言語の型のトークンに対応する型に変換する
///
/// # Arguments
/// - `token_type`: 変換するトークン
///
/// # Returns
/// - `Type`: 変換された型
/// - `ParserError::TypeNotFound`: 型が見つからなかった場合
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
