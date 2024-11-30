use crate::virtual_machine::ast::Type;
// cfg(test)を使っているので、テスト時のみ使用される
#[allow(unused_imports)]
use crate::virtual_machine::parser::{Parser, ParserError};
#[allow(unused_imports)]
use crate::virtual_machine::token::token_type::TokenType;
#[allow(unused_imports)]
use crate::virtual_machine::token::Token;

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

/// Parserを指定のトークン列で初期化するテスト用の関数
///
/// # Arguments
/// - `tokens`: 初期化するトークン列
///
/// # Returns
/// - `Parser`: 生成されたParser。Token列の後ろにEOFが追加されている。
#[cfg(test)]
pub fn create_parser_with_tokens(tokens: Vec<Token>) -> Parser {
    let mut tokens_with_eof: Vec<Token> = tokens;
    tokens_with_eof.push(Token::new(1, 1, TokenType::Eof)); // EOFを追加
    Parser::new(tokens_with_eof)
}
