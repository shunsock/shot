pub mod parse_binary;
pub mod parse_call_of_variable_and_function;
pub mod parse_parenthesized;
pub mod parse_primary;
mod parse_type_cast;

use crate::virtual_machine::ast::ExpressionNode;
use crate::virtual_machine::parser::expression_parser::parse_type_cast::parse_type_cast;
use crate::virtual_machine::parser::parser_error::ParserError;
use crate::virtual_machine::parser::Parser;
use crate::virtual_machine::token::token_type::TokenType;
use parse_binary::parse_binary;

/// 式のパース
///
/// # Returns
/// - `Result<ExpressionNode, ParserError>`: 式
///
/// # Syntax
/// 式は、計算式や関数呼び出し、変数の参照などが含まれます
///
/// `ExpressionNode`を返します
///
/// ## Example
/// - 二項演算式: `1 + 2`
/// - 関数呼び出し: `f()`
/// - 変数参照: `x`
pub fn parse_expression(parser: &mut Parser) -> Result<ExpressionNode, ParserError> {
    let expr: ExpressionNode = parse_binary(parser)?;

    match parser.peek().token_type.clone() {
        // TypeCastの対象となっている場合は、
        // ExpressionNode::TypeCastNode { from_type, to_type, expression } に変換する
        // ex: `Expression as from_type -> to_type;`
        TokenType::As => parse_type_cast(parser, expr),
        // それ以外はそのまま返す
        _ => Ok(expr),
    }
}
