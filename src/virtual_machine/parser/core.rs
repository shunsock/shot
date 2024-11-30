use crate::virtual_machine::ast::Type;
// cfg(test)を使っているので、テスト時のみ使用される
#[allow(unused_imports)]
use crate::virtual_machine::parser::{Parser, ParserError};
#[allow(unused_imports)]
use crate::virtual_machine::token::token_type::TokenType;
#[allow(unused_imports)]
use crate::virtual_machine::token::Token;

/// パーサーの現在のトークンから型情報を取得する
///
/// # Arguments
/// - `parser`: Parser
///
/// # Returns
/// - `Type`: 変換された型
/// - `ParserError::TypeNotFound`: 型が見つからなかった場合
///
/// # Raises
/// - `ParserError::TypeNotFound`: 型が見つからなかった場合
pub fn get_type_from_current_token(parser: &mut Parser) -> Result<Type, ParserError> {
    let current_token: Token = parser.peek().clone();
    match current_token.token_type.clone() {
        TokenType::IntType => Ok(Type::Integer),
        TokenType::FloatType => Ok(Type::Float),
        TokenType::StringType => Ok(Type::String),
        TokenType::VoidType => Ok(Type::Void),
        TokenType::Fn => Ok(Type::Function),
        _ => Err(ParserError::TypeNotFound {
            found: current_token.token_type.clone(),
            line: current_token.line,
            char_pos: current_token.char_pos,
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
