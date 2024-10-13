use crate::virtual_machine::ast::{ExpressionNode, LiteralNode, LiteralValue};
use crate::virtual_machine::parser::parser_error::ParserError;
use crate::virtual_machine::parser::Parser;
use crate::virtual_machine::token::token_type::TokenType;

pub fn parse_primary(parser: &mut Parser) -> Result<ExpressionNode, ParserError> {
    match parser.peek().token_type.clone() {
        // 整数リテラル
        TokenType::IntegerLiteral(value) => {
            parser.advance();
            Ok(ExpressionNode::Literal(Box::new(LiteralNode {
                value: LiteralValue::Integer(value),
            })))
        }
        // 浮動小数点リテラル
        TokenType::FloatLiteral(value) => {
            parser.advance();
            Ok(ExpressionNode::Literal(Box::new(LiteralNode {
                value: LiteralValue::Float(value),
            })))
        }
        // 文字列リテラル
        TokenType::StringLiteral(ref value) => {
            parser.advance();
            Ok(ExpressionNode::Literal(Box::new(LiteralNode {
                value: LiteralValue::String(value.clone()),
            })))
        }
        // Noneリテラル
        TokenType::NoneLiteral => {
            parser.advance();
            Ok(ExpressionNode::Literal(Box::new(LiteralNode {
                value: LiteralValue::None,
            })))
        }
        // 他のリテラルが必要な場合に追加 (例: NoneLiteral)
        _ => Err(ParserError::UnexpectedTokenType {
            token: parser.peek().token_type.clone(),
            line: parser.peek().line,
            char_pos: parser.peek().char_pos,
        }),
    }
}
