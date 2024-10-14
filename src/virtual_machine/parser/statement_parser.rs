use crate::virtual_machine::ast::{ExpressionNode, Statement};
use crate::virtual_machine::parser::expression_parser::parse_expression;
use crate::virtual_machine::parser::Parser;
use crate::virtual_machine::parser::ParserError;
use crate::virtual_machine::token::token_type::TokenType;

pub fn parse_statement(parser: &mut Parser) -> Result<Statement, ParserError> {
    match parser.peek().token_type.clone() {
        TokenType::Let => {
            // 仮実装: 変数宣言のパース
            let token = parser.peek().clone();
            Err(ParserError::NotImplementedError {
                feature: "Variable declaration".to_string(),
                line: token.line,
                char_pos: token.char_pos,
            })
        }
        TokenType::Fn => {
            // 仮実装: 関数宣言のパース
            let token = parser.peek().clone();
            Err(ParserError::NotImplementedError {
                feature: "Function declaration".to_string(),
                line: token.line,
                char_pos: token.char_pos,
            })
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

fn parse_return_statement(parser: &mut Parser) -> Result<Statement, ParserError> {
    // returnキーワードを読み飛ばす
    parser.advance();

    // 式をパース
    let expr: ExpressionNode = parse_expression(parser)?;

    parser.expect(TokenType::Semicolon)?;

    Ok(Statement::Return(Box::new(expr)))
}

fn parse_expression_statement(parser: &mut Parser) -> Result<Statement, ParserError> {
    // 式をパース
    let expr: ExpressionNode = parse_expression(parser)?;

    // 式文の終端にセミコロンがあることを確認
    parser.expect(TokenType::Semicolon)?;

    Ok(Statement::Expression(expr))
}
