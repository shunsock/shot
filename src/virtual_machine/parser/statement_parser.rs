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

/// ## 宣言文のパース
///
/// 宣言文は、変数や関数を宣言する際に用いる文です
///
/// ## Syntax
///
/// let x: int = *Expression Node*;
/// let f: fn = () => { *Statements* };
fn parse_let_statement(parser: &mut Parser) -> Result<Statement, ParserError> {
    // letキーワードを読み飛ばす
    parser.advance();

    // 宣言文をパース
    let statement: Statement = parse_declaration(parser)?;

    // 終端にセミコロンがあることを確認
    parser.check_advance(TokenType::Semicolon)?;

    Ok(statement)
}

/// ## Return文のパース
///
/// Return文は関数内で値を返す際に用いる文です
///
/// ## Syntax
///
/// return *Expression Node*;
fn parse_return_statement(parser: &mut Parser) -> Result<Statement, ParserError> {
    // returnキーワードを読み飛ばす
    parser.advance();

    // 式をパース
    let expr: ExpressionNode = parse_expression(parser)?;

    // 終端にセミコロンがあることを確認
    parser.check_advance(TokenType::Semicolon)?;

    Ok(Statement::Return(Box::new(expr)))
}

/// # 式文（Expression Statement）のパース
///
/// 式文は、式の末尾にセミコロンがついたものです。
///
/// ## Syntax
///
/// *Expression Node*;
fn parse_expression_statement(parser: &mut Parser) -> Result<Statement, ParserError> {
    // 式をパース
    let expr: ExpressionNode = parse_expression(parser)?;

    // 終端にセミコロンがあることを確認
    parser.check_advance(TokenType::Semicolon)?;

    Ok(Statement::Expression(expr))
}
