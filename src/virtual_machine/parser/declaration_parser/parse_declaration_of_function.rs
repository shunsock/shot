use shot::virtual_machine::parser::parser_error::ParserError::MismatchedToken;
use shot::virtual_machine::token::token_type::TokenType;
use crate::virtual_machine::ast::FunctionDeclarationNode;
use crate::virtual_machine::ast::{Statement, Type};
use crate::virtual_machine::parser::parser_error::ParserError;
use crate::virtual_machine::parser::Parser;

/// ## 関数宣言をパースする関数
///
/// ## Example
/// let f: fn = () => { *Statements* };
/// let tokenは上流の `declaration_parser.rs` の `parse_declaration` で消費されている
/// semicolon tokenは上流の `statement_parser.rs` の `parse_statement` で消費される
/// この関数では、 f: fn = () => { *Statements* } の部分をパースする
pub(crate) fn parse_declaration_of_function(parser: &mut Parser) -> Result<Statement, ParserError> {
    let name: String = match parser.peek().token_type.clone() {
        TokenType::Identifier(name) => name,
        token_type => return Err(MismatchedToken {
            expected: TokenType::Identifier(String::from("function_name")),
            found: token_type,
            line: 0,
            char_pos: 0,
        })
    };

    // Identifier(名前)を読み飛ばす
    parser.advance();

    // Colonがあることを確認して読み飛ばす
    parser.check_advance(TokenType::Colon)?;

    // Fnがあることを確認して読み飛ばす
    parser.check_advance(TokenType::Fn)?;

    // Equalがあることを確認して読み飛ばす
    parser.check_advance(TokenType::Equal)?;

    // LeftParenがあることを確認して読み飛ばす
    parser.check_advance(TokenType::LeftParen)?;

    // 引数を確認する
    let parameters: Vec<(String, Type)> = match parser.peek().token_type.clone() {
        TokenType::RightParen => vec![],
        _ => parse_parameters(parser)?
    };

    // RightParenがあることを確認して読み飛ばす
    parser.check_advance(TokenType::RightParen)?;

    // Statementsを確認する


    Ok(Statement::DeclarationOfFunction(Box::new(
        FunctionDeclarationNode {
            name,
            params: vec![],
            return_type: Type::Integer,
            body: vec![],
        },
    )))
}

fn parse_parameters(parser: &mut Parser) -> Result<Vec<(String, Type)>, ParserError> {
    let mut parameters: Vec<(String, Type)> = vec![];
    loop {
        let parameter_name: TokenType::Identifier = match parser.peek().token_type.clone() {
            TokenType::Identifier(parameter_name) => parameter_name,
            token => return Err(MismatchedToken {
                expected: TokenType::Identifier(String::from("parameter_name")),
                found: token.clone(),
                line: parser.peek().line,
                char_pos: parser.peek().char_pos,
            })
        };
        parser.advance();
        
        // 次のTokenTypeがRightParenならLoopを抜ける
        if parser.peek().token_type.clone() { break; }

        // 引数の後にはコロンと型情報が入る
        // f(x: int ...
        parser.check_advance(TokenType::Colon)?;
        let parameter_type: Type = match parser.peek().token_type.clone() {
            TokenType::IntType => Type::Integer,
            TokenType::FloatType => Type::Float,
            TokenType::StringType => Type::String,
            TokenType::VoidType => Type::Void,
            TokenType::Fn => Type::Function,
            token => return Err(MismatchedToken {
                expected: TokenType::Identifier(String::from("parameter_name")),
                found: token.clone(),
                line: parser.peek().line,
                char_pos: parser.peek().char_pos,
            })
        };

        // パラメータの間にはカンマが入る
        // f(x: int, ...
        parser.check_advance(TokenType::Comma)?;

        // 引数と型の組みをpush
        parameters.push((parameter_name, parameter_type));
    }
    Ok(parameters)
}
