use crate::virtual_machine::ast::{ExpressionNode, Statement};
use crate::virtual_machine::parser::declaration_parser::parse_declaration;
use crate::virtual_machine::parser::expression_parser::parse_expression;
use crate::virtual_machine::parser::Parser;
use crate::virtual_machine::parser::ParserError;
use crate::virtual_machine::token::token_type::TokenType;

pub fn parse_statement(parser: &mut Parser) -> Result<Statement, ParserError> {
    match parser.peek().token_type.clone() {
        TokenType::Let => {
            // let文のパース
            parse_let_statement(parser)
        }
        TokenType::Return => {
            // return文のパース
            parse_return_statement(parser)
        }
        _ => {
            // 式文（Expression Statement）のパース
            parse_expression_statement(parser)
        }
    }
}

fn parse_let_statement(parser: &mut Parser) -> Result<Statement, ParserError> {
    parse_declaration(parser)
}

fn parse_return_statement(parser: &mut Parser) -> Result<Statement, ParserError> {
    // returnキーワードを読み飛ばす
    parser.advance();

    // 式をパース
    let expr: ExpressionNode = parse_expression(parser)?;

    parser.check_advance(TokenType::Semicolon)?;

    Ok(Statement::Return(Box::new(expr)))
}

/// # 式文（Expression Statement）のパース
///
/// 式文は、式の末尾にセミコロンがついたものです。
///
/// ## Syntax
///
/// ```text
/// expression_statement = expression_node ";"
/// ```
///
/// ## Note
///
/// 式文は下記のような定義になっています。
///
/// ```
/// pub enum ExpressionNode {
//     BinaryOperation(Box<BinaryOperationNode>), // 二項演算
//     CallOfFunction(Box<FunctionCallNode>),     // 関数呼び出し
//     CallOfVariable(Box<VariableCallNode>),     // 識別子
//     Literal(Box<LiteralNode>),                 // リテラル
//     TypeCast(Box<TypeCastNode>),               // 型キャスト
/// }
/// ```
///
/// このうち `BinaryOperation` などの式は、内部にさらに式を持つことがあります。
fn parse_expression_statement(parser: &mut Parser) -> Result<Statement, ParserError> {
    // 式をパース
    let expr: ExpressionNode = parse_expression(parser)?;

    // 式文の終端にセミコロンがあることを確認
    parser.check_advance(TokenType::Semicolon)?;

    Ok(Statement::Expression(expr))
}
