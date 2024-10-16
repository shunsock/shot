use crate::virtual_machine::ast::{ExpressionNode, FunctionCallNode, VariableCallNode};
use crate::virtual_machine::parser::expression_parser::parse_expression;
use crate::virtual_machine::parser::parser_error::ParserError;
use crate::virtual_machine::parser::Parser;
use crate::virtual_machine::token::token_type::TokenType;

pub fn parse_identifier_or_call(parser: &mut Parser) -> Result<ExpressionNode, ParserError> {
    let name: String = match parser.peek().token_type.clone() {
        TokenType::Identifier(name) => name,
        _ => {
            return Err(ParserError::UnexpectedTokenType {
                token: parser.peek().token_type.clone(),
                line: parser.peek().line,
                char_pos: parser.peek().char_pos,
            })
        }
    };
    // 名前を読み飛ばす
    // Function: let f: fn(): void = { ... } のf
    // Variable Declaration: let x: int = 0; のx
    parser.advance();

    // 次のトークンが左括弧なら関数呼び出し
    match parser.peek().token_type {
        TokenType::LeftParen => {
            // LeftParenを読み飛ばす
            parser.advance();

            // 引数を読み込む
            let mut args: Vec<ExpressionNode> = Vec::new();
            while !parser.check(TokenType::RightParen) {
                match parser.peek().token_type.clone() {
                    TokenType::Comma => {
                        // カンマがあれば読み飛ばす
                        parser.advance();
                    }
                    _ => {
                        // 引数を読み込む
                        args.push(parse_expression(parser)?);
                    }
                };
            }

            // 右括弧があることを確認して読み飛ばす
            parser.check_advance(TokenType::RightParen)?;

            Ok(ExpressionNode::CallOfFunction(Box::new(FunctionCallNode {
                name: name.clone(),
                arguments: args,
            })))
        }
        _ => {
            // 変数呼び出し
            Ok(ExpressionNode::CallOfVariable(Box::new(VariableCallNode {
                name: name.clone(),
            })))
        }
    }
}
