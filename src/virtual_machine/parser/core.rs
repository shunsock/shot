use crate::virtual_machine::ast::Precedence;
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

/// 演算子に対応する優先度を返す関数
pub(crate) fn get_precedence(token_type: &TokenType) -> Precedence {
    match token_type {
        TokenType::Plus | TokenType::Minus => Precedence::Low, // 加算、減算は低い優先順位
        TokenType::Asterisk | TokenType::Slash => Precedence::High, // 乗算、除算は高い優先順位
        _ => Precedence::Low,                                  // その他は低い優先順位
    }
}
