use crate::virtual_machine::ast::VariableDeclarationNode;
use crate::virtual_machine::ast::{ExpressionNode, Statement, Type};
use crate::virtual_machine::parser::core::type_token_to_type;
use crate::virtual_machine::parser::expression_parser::parse_expression;
use crate::virtual_machine::parser::Parser;
use crate::virtual_machine::parser::ParserError;
use crate::virtual_machine::token::token_type::TokenType;

pub(crate) fn parse_declaration(parser: &mut Parser) -> Result<Statement, ParserError> {
    // letキーワードを読み飛ばす
    parser.advance();

    // 変数宣言と関数宣言をRoute
    match parser.peek_next().token_type {
        TokenType::Colon => parse_declaration_of_variable(parser),
        TokenType::LeftParen => parse_declaration_of_function(parser),
        _ => Err(ParserError::MismatchedToken {
            expected: TokenType::Equal,
            found: parser.peek().token_type.clone(),
            line: parser.peek().line,
            char_pos: parser.peek().char_pos,
        }),
    }
}

fn parse_declaration_of_variable(parser: &mut Parser) -> Result<Statement, ParserError> {
    // 名前を読み取る
    let name: String = match parser.peek().token_type.clone() {
        TokenType::Identifier(name) => name,
        _ => {
            return Err(ParserError::MismatchedToken {
                expected: TokenType::Identifier("variable name".to_string()),
                found: parser.peek().token_type.clone(),
                line: parser.peek().line,
                char_pos: parser.peek().char_pos,
            })
        }
    };
    parser.advance();

    // colonを読み飛ばす
    parser.expect(TokenType::Colon)?;

    // 型を読み取る
    let type_token: TokenType = parser.peek().token_type.clone();
    let variable_type: Type = type_token_to_type(type_token)?;
    parser.advance();

    // イコールを読み飛ばす
    parser.expect(TokenType::Equal)?;

    // 式をパース
    let expr: ExpressionNode = parse_expression(parser)?;

    // セミコロンを読み飛ばす
    parser.expect(TokenType::Semicolon)?;

    Ok(Statement::DeclarationOfVariable(Box::new(
        VariableDeclarationNode {
            name,
            var_type: variable_type,
            value: Box::new(expr),
        },
    )))
}

fn parse_declaration_of_function(parser: &mut Parser) -> Result<Statement, ParserError> {
    Err(ParserError::NotImplementedError {
        feature: "Function declaration".to_string(),
        line: parser.peek().line,
        char_pos: parser.peek().char_pos,
    })
}
