use crate::virtual_machine::ast::{ExpressionNode, Type, TypeCastNode};
use crate::virtual_machine::parser::core::type_token_to_type;
use crate::virtual_machine::parser::parser_error::ParserError;
use crate::virtual_machine::parser::Parser;
use crate::virtual_machine::token::token_type::TokenType;

/// ## Parse a type cast expression.
///
/// ## Syntax
///
/// 1 as int -> string;
pub fn parse_type_cast(
    parser: &mut Parser,
    expr: ExpressionNode,
) -> Result<ExpressionNode, ParserError> {
    // asを確認して読み飛ばす
    parser.check_advance(TokenType::As)?;

    // 型を読み取る
    let from_type: Type = type_token_to_type(parser.peek().token_type.clone())?;
    parser.advance();

    // -> を確認して読み飛ばす
    parser.check_advance(TokenType::TypeCastArrow)?;

    // 型を読み取る
    let to_type: Type = type_token_to_type(parser.peek().token_type.clone())?;
    parser.advance();

    // 式を返す
    Ok(ExpressionNode::TypeCast(Box::new(TypeCastNode {
        from_type,
        to_type,
        expression: Box::new(expr),
    })))
}
