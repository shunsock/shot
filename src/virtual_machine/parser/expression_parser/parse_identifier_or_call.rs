use crate::virtual_machine::ast::{ExpressionNode, FunctionCallNode, VariableCallNode};
use crate::virtual_machine::parser::expression_parser::parse_expression;
use crate::virtual_machine::parser::parser_error::ParserError;
use crate::virtual_machine::parser::Parser;
use crate::virtual_machine::token::token_type::TokenType;

pub fn parse_identifier_or_call(parser: &mut Parser) -> Result<ExpressionNode, ParserError> {
    if let TokenType::Identifier(ref name) = parser.peek().token_type.clone() {
        parser.advance();

        // 次のトークンが左括弧なら関数呼び出し
        if parser.match_token(TokenType::LeftParen) {
            let mut args = Vec::new();
            while !parser.check(TokenType::RightParen) {
                args.push(parse_expression(parser)?);
                if !parser.match_token(TokenType::Comma) {
                    break;
                }
            }
            parser.expect(TokenType::RightParen)?;
            Ok(ExpressionNode::CallOfFunction(Box::new(FunctionCallNode {
                name: name.clone(),
                arguments: args,
            })))
        } else {
            // 変数呼び出し
            Ok(ExpressionNode::CallOfVariable(Box::new(VariableCallNode {
                name: name.clone(),
            })))
        }
    } else {
        Err(ParserError::UnexpectedTokenType {
            token: parser.peek().token_type.clone(),
            line: parser.peek().line,
            char_pos: parser.peek().char_pos,
        })
    }
}
