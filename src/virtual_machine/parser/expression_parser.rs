use crate::virtual_machine::ast::{ExpressionNode, LiteralNode, LiteralValue};
use crate::virtual_machine::parser::parser_error::ParserError;
use crate::virtual_machine::parser::Parser;
use crate::virtual_machine::token::token_type::TokenType;

pub fn parse_expression(parser: &mut Parser) -> Result<ExpressionNode, ParserError> {
    parse_primary(parser)
}

fn parse_primary(parser: &mut Parser) -> Result<ExpressionNode, ParserError> {
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
        // 他の式（関数呼び出し、変数参照、二項演算など）は未実装
        TokenType::Identifier(_) => {
            let token = parser.peek().clone();
            Err(ParserError::NotImplementedError {
                feature: "Variable call".to_string(),
                line: token.line,
                char_pos: token.char_pos,
            })
        }
        TokenType::LeftParen => {
            let token = parser.peek().clone();
            Err(ParserError::NotImplementedError {
                feature: "Parenthesized expression".to_string(),
                line: token.line,
                char_pos: token.char_pos,
            })
        }
        // 不明なトークン
        _ => Err(ParserError::UnexpectedTokenType {
            token: parser.peek().token_type.clone(),
            line: parser.peek().line,
            char_pos: parser.peek().char_pos,
        }),
    }
}
