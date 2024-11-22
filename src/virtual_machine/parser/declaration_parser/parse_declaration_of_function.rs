use crate::virtual_machine::ast::FunctionDeclarationNode;
use crate::virtual_machine::ast::{Statement, Type};
use crate::virtual_machine::parser::parser_error::ParserError;
use crate::virtual_machine::parser::parser_error::ParserError::MismatchedToken;
use crate::virtual_machine::parser::parser_error::ParserError::UnexpectedEof;
use crate::virtual_machine::parser::statement_parser::parse_statement;
use crate::virtual_machine::parser::Parser;
use crate::virtual_machine::token::token_type::TokenType;

/// ## 関数宣言をパースする関数
///
/// ## Example
/// let f: fn = (): type => { *Statements* };
/// let tokenは上流の `declaration_parser.rs` の `parse_declaration` で消費されている
/// semicolon tokenは上流の `statement_parser.rs` の `parse_statement` で消費される
/// この関数では、 f: fn = (): type => { *Statements* } の部分をパースする
pub(crate) fn parse_declaration_of_function(parser: &mut Parser) -> Result<Statement, ParserError> {
    let name: String = match parser.peek().token_type.clone() {
        TokenType::Identifier(name) => name,
        token_type => {
            return Err(MismatchedToken {
                expected: TokenType::Identifier(String::from("function_name")),
                found: token_type,
                line: 0,
                char_pos: 0,
            })
        }
    };

    // Identifier(名前)を読み飛ばす: let f
    parser.advance();

    // Colonがあることを確認して読み飛ばす: let f:
    parser.check_advance(TokenType::Colon)?;

    // Fnがあることを確認して読み飛ばす: let f: fn
    parser.check_advance(TokenType::Fn)?;

    // Equalがあることを確認して読み飛ばす: let f: fn =
    parser.check_advance(TokenType::Equal)?;

    // LeftParenがあることを確認して読み飛ばす: let f: fn = (
    parser.check_advance(TokenType::LeftParen)?;

    // 引数を確認する: let f: fn = (x: int, y: string
    let params: Vec<(String, Type)> = match parser.peek().token_type.clone() {
        TokenType::RightParen => vec![],
        _ => parse_parameters(parser)?,
    };

    // RightParenがあることを確認して読み飛ばす
    // let f: fn = (x: int, y: float)
    parser.check_advance(TokenType::RightParen)?;

    // Colonがあることを確認して読み飛ばす
    // let f: fn = (x: int, y: float):
    parser.check_advance(TokenType::Colon)?;

    // 戻り値の型を確認する
    // let f: fn = (x: int, y: float): string
    let return_type: Type = get_type(parser)?;
    parser.advance();

    // 左波括弧があることを確認して読み飛ばす
    // let f: fn = (x: int, y: float): string {
    parser.check_advance(TokenType::LeftBrace)?;

    // 関数の中身をパースする
    // let f: fn = (x: int, y: float): string { ...
    let body: Vec<Statement> = parse_function_body(parser)?;

    // 右波括弧があることを確認して読み飛ばす
    // let f: fn = (x: int, y: float): string { ... }
    parser.check_advance(TokenType::RightBrace)?;

    Ok(Statement::DeclarationOfFunction(Box::new(
        FunctionDeclarationNode {
            name,
            params,
            return_type,
            body,
        },
    )))
}

fn parse_parameters(parser: &mut Parser) -> Result<Vec<(String, Type)>, ParserError> {
    let mut parameters: Vec<(String, Type)> = vec![];
    loop {
        let parameter_name: String = match parser.peek().token_type.clone() {
            TokenType::Identifier(parameter_name) => parameter_name,
            token => {
                return Err(MismatchedToken {
                    expected: TokenType::Identifier(String::from("parameter_name")),
                    found: token.clone(),
                    line: parser.peek().line,
                    char_pos: parser.peek().char_pos,
                })
            }
        };
        parser.advance();

        // 次のTokenTypeがRightParenならLoopを抜ける
        if parser.peek().token_type.clone() == TokenType::RightParen {
            break;
        }

        // 引数の後にはコロンと型情報が入る
        // f(x: int ...
        parser.check_advance(TokenType::Colon)?;

        // 型情報を取得
        let parameter_type: Type = get_type(parser)?;

        // パラメータの間にはカンマが入る
        // f(x: int, ...
        parser.check_advance(TokenType::Comma)?;

        // 引数と型の組みをpush
        parameters.push((parameter_name, parameter_type));
    }
    Ok(parameters)
}

fn parse_function_body(parser: &mut Parser) -> Result<Vec<Statement>, ParserError> {
    let mut statements: Vec<Statement> = vec![];
    loop {
        let token: TokenType = parser.peek().token_type.clone();
        match token {
            TokenType::RightBrace => {
                return Err(MismatchedToken {
                    expected: TokenType::Return,
                    found: token.clone(),
                    line: parser.peek().line,
                    char_pos: parser.peek().char_pos,
                })
            }
            TokenType::Eof => {
                return Err(UnexpectedEof {
                    expected: TokenType::Return,
                    line: parser.peek().line,
                    char_pos: parser.peek().char_pos,
                })
            }
            TokenType::Return => {
                let statement: Statement = parse_statement(parser)?;
                statements.push(statement);

                // Return文で関数は終わる
                break;
            }
            _ => {
                let statement: Statement = parse_statement(parser)?;
                statements.push(statement);
            }
        }
    }
    Ok(statements)
}

fn get_type(parser: &mut Parser) -> Result<Type, ParserError> {
    let token: TokenType = parser.peek().token_type.clone();
    match token {
        TokenType::IntType => Ok(Type::Integer),
        TokenType::FloatType => Ok(Type::Float),
        TokenType::StringType => Ok(Type::String),
        TokenType::VoidType => Ok(Type::Void),
        TokenType::Fn => Ok(Type::Function),
        token => Err(MismatchedToken {
            expected: TokenType::Identifier(String::from("parameter_name")),
            found: token.clone(),
            line: parser.peek().line,
            char_pos: parser.peek().char_pos,
        }),
    }
}
