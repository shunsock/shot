use crate::virtual_machine::ast::{ExpressionNode, Statement};
use crate::virtual_machine::parser::declaration_parser::parse_declaration;
use crate::virtual_machine::parser::expression_parser::parse_expression;
use crate::virtual_machine::parser::Parser;
use crate::virtual_machine::parser::ParserError;
use crate::virtual_machine::token::token_type::TokenType;

/// 文のパース
///
/// # Syntax
/// 文は、一単位です。let文やreturn文、式文などが含まれます
///
/// ## Example
/// - let文: `let x: int = 0;`
/// - return文: `return 0;`
/// - expression文: `0;`
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
            // Expression文のパース
            parse_expression_statement(parser)
        }
    }
}

/// 宣言文のパース
///
/// # Syntax
/// 宣言文は、変数や関数を宣言する際に用いる文です
///
/// ## Example
/// - 変数宣言: `let x: int = *Expression Node*;`
/// - 関数宣言: `let f: fn = () => { *Statements* };`
fn parse_let_statement(parser: &mut Parser) -> Result<Statement, ParserError> {
    // letキーワードを読み飛ばす
    parser.advance();

    // 宣言文をパース
    let statement: Statement = parse_declaration(parser)?;

    // 終端にセミコロンがあることを確認
    parser.check_advance(TokenType::Semicolon)?;

    Ok(statement)
}

/// 戻り値を返す文のパース
///
/// ## Syntax
/// 戻り値を返す文は関数内で値を返す際に用いる文です
///
/// ## Example
/// - `return *Expression Node*;`
fn parse_return_statement(parser: &mut Parser) -> Result<Statement, ParserError> {
    // returnキーワードを読み飛ばす
    parser.advance();

    // 式をパース
    let expr: ExpressionNode = parse_expression(parser)?;

    // 終端にセミコロンがあることを確認
    parser.check_advance(TokenType::Semicolon)?;

    Ok(Statement::Return(Box::new(expr)))
}

/// 式文のパース
///
/// ## Syntax
/// 式文は、式の末尾にセミコロンがついたものです。
///
/// ## Example
/// *Expression Node*;
fn parse_expression_statement(parser: &mut Parser) -> Result<Statement, ParserError> {
    // 式をパース
    let expr: ExpressionNode = parse_expression(parser)?;

    // 終端にセミコロンがあることを確認
    parser.check_advance(TokenType::Semicolon)?;

    Ok(Statement::Expression(expr))
}
