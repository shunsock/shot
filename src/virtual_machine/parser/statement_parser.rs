use crate::virtual_machine::ast::{ExpressionNode, Statement, Type, VariableDeclarationNode};
use crate::virtual_machine::parser::core::type_token_to_type;
use crate::virtual_machine::parser::declaration_parser::parse_declaration;
use crate::virtual_machine::parser::expression_parser::parse_expression;
use crate::virtual_machine::parser::Parser;
use crate::virtual_machine::parser::ParserError;
use crate::virtual_machine::token::token_type::TokenType;
use crate::virtual_machine::token::Token;
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

fn parse_expression_statement(parser: &mut Parser) -> Result<Statement, ParserError> {
    // 式をパース
    let expr: ExpressionNode = parse_expression(parser)?;

    // 式文の終端にセミコロンがあることを確認
    parser.check_advance(TokenType::Semicolon)?;

    Ok(Statement::Expression(expr))
}
