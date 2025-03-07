use crate::virtual_machine::ast::{ExpressionNode, FunctionCallNode, VariableCallNode};
use crate::virtual_machine::parser::expression_parser::parse_expression;
use crate::virtual_machine::parser::parser_error::ParserError;
use crate::virtual_machine::parser::Parser;
use crate::virtual_machine::token::token_type::TokenType;

/// 関数呼び出しと変数参照のパース
///
/// # Return
/// - `Result<ExpressionNode, ParserError>` パース結果
///   - `ExpressionNode`: ExpressionのAST Node
///   - `ParserError`: Parserのエラー
///
/// # Syntax
/// 関数呼び出しは、expressionの一種で、以下のように定義される
/// - 関数呼び出し: `name ( [arguments] )` where `arguments` is `name: expression [, name: expression]*`
///
/// 変数参照は、expressionの一種で、以下のように定義される
/// - 変数参照: `name`
///
/// # Example
/// - 関数呼び出し: `f()`, `f(x: 0)`, `f(x: 0, y: "shunsock")`
/// - 変数参照: `x`
pub fn parse_identifier_or_call(parser: &mut Parser) -> Result<ExpressionNode, ParserError> {
    let name: String = match parser.peek().token_type.clone() {
        TokenType::Identifier(name) => name,
        _ => {
            return Err(ParserError::UnexpectedTokenType {
                token: parser.peek().token_type.clone(),
                line: parser.peek().line,
                char_pos: parser.peek().char_pos,
            })
        }
    };

    // 名前を読み飛ばす
    // Function: let f: fn(): void = { ... } のf
    // Variable Declaration: let x: int = 0; のx
    parser.advance();

    match parser.peek().token_type {
        // 次のトークンが左括弧なら関数呼び出し
        TokenType::LeftParen => parse_call_of_function(name.clone(), parser),
        // それ以外であれば変数の呼び出し
        _ => parse_call_of_variable(name.clone()),
    }
}

fn parse_call_of_function(
    name: String,
    parser: &mut Parser,
) -> Result<ExpressionNode, ParserError> {
    // LeftParenを読み飛ばす
    parser.advance();

    // 引数がない場合のearly return
    if parser.check(TokenType::RightParen) {
        // 右括弧があることを確認して読み飛ばす
        parser.advance();
        return Ok(ExpressionNode::CallOfFunction(Box::new(FunctionCallNode {
            name: name.clone(),
            arguments: vec![],
        })));
    }

    // 引数がある場合の処理
    let mut args: Vec<(String, ExpressionNode)> = Vec::new();
    loop {
        let argument_name = match parser.peek().token_type.clone() {
            TokenType::Identifier(name) => name,
            _ => {
                return Err(ParserError::UnexpectedTokenType {
                    token: parser.peek().token_type.clone(),
                    line: parser.peek().line,
                    char_pos: parser.peek().char_pos,
                })
            }
        };
        // 引数名を読み飛ばす
        parser.advance();

        // ":"を読み飛ばす
        parser.check_advance(TokenType::Colon)?;

        // 引数を読み込む
        args.push((argument_name, parse_expression(parser)?));

        // 次が ")" なら処理終了
        if parser.check(TokenType::RightParen) {
            break;
        }

        // そうでないならば次の引数が存在する
        // f(a, <- 次の引数が来るはず
        // ","があることを確認して次のループへ行く
        parser.check_advance(TokenType::Comma)?;
    }

    // 右括弧があることを確認して読み飛ばす
    parser.check_advance(TokenType::RightParen)?;

    Ok(ExpressionNode::CallOfFunction(Box::new(FunctionCallNode {
        name: name.clone(),
        arguments: args,
    })))
}

fn parse_call_of_variable(name: String) -> Result<ExpressionNode, ParserError> {
    Ok(ExpressionNode::CallOfVariable(Box::new(VariableCallNode {
        name,
    })))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::virtual_machine::ast::{LiteralNode, LiteralValue};
    use crate::virtual_machine::parser::core::create_parser_with_tokens;
    use crate::virtual_machine::parser::Parser;
    use crate::virtual_machine::token::{token_type::TokenType, Token};

    // 正常系テスト

    /// 引数が存在しない関数のパースが可能か確認するテスト
    /// f();
    #[test]
    fn parse_function_without_arguments() {
        // 生成されるAST Node
        let expected: Box<FunctionCallNode> = Box::new(FunctionCallNode {
            name: "f".to_string(),
            arguments: vec![],
        });

        // テストする関数の入力である、Token列, Parserの生成
        // f();
        let tokens: Vec<Token> = vec![
            Token::new(1, 1, TokenType::Identifier("f".to_string())),
            Token::new(1, 2, TokenType::LeftParen),
            Token::new(1, 3, TokenType::RightParen),
            Token::new(1, 4, TokenType::Semicolon),
        ];
        let mut parser: Parser = create_parser_with_tokens(tokens);

        // テストしたい関数の出力 (エラーが出ていないことを確認)
        let result: Result<ExpressionNode, ParserError> = parse_identifier_or_call(&mut parser);
        assert!(result.is_ok());

        // テストしたい関数の出力と期待値を比較
        let call_of_function: Box<FunctionCallNode> = match result.clone().unwrap() {
            ExpressionNode::CallOfFunction(call_of_function) => call_of_function,
            _ => panic!("ExpectedNode::CallOfFunction"),
        };

        assert_eq!(call_of_function, expected);
    }

    /// 引数が一つ存在する関数のパースが可能か確認するテスト
    /// f(v: 0);
    #[test]
    fn parse_function_with_an_argument() {
        // 生成されるAST Node
        let expected: Box<FunctionCallNode> = Box::new(FunctionCallNode {
            name: "f".to_string(),
            arguments: vec![(
                "v".to_string(),
                ExpressionNode::Literal(Box::new(LiteralNode {
                    value: LiteralValue::Integer(0),
                })),
            )],
        });

        // テストする関数の入力である、Token列, Parserの生成
        // f(v: 0);
        let tokens: Vec<Token> = vec![
            Token::new(1, 1, TokenType::Identifier("f".to_string())),
            Token::new(1, 2, TokenType::LeftParen),
            Token::new(1, 1, TokenType::Identifier("v".to_string())),
            Token::new(1, 2, TokenType::Colon),
            Token::new(1, 3, TokenType::IntegerLiteral(0)),
            Token::new(1, 3, TokenType::RightParen),
            Token::new(1, 4, TokenType::Semicolon),
        ];
        let mut parser: Parser = create_parser_with_tokens(tokens);

        // テストしたい関数の出力 (エラーが出ていないことを確認)
        let result: Result<ExpressionNode, ParserError> = parse_identifier_or_call(&mut parser);
        assert!(result.is_ok());

        // テストしたい関数の出力と期待値を比較
        let call_of_function: Box<FunctionCallNode> = match result.clone().unwrap() {
            ExpressionNode::CallOfFunction(call_of_function) => call_of_function,
            _ => panic!("ExpectedNode::CallOfFunction"),
        };
        assert_eq!(call_of_function, expected);
    }

    /// 引数が複数存在する関数のパースが可能か確認するテスト
    /// f(x: 0, y: "shunsock");
    #[test]
    fn parse_function_with_arguments() {
        // 生成されるAST Node
        let expected: Box<FunctionCallNode> = Box::new(FunctionCallNode {
            name: "f".to_string(),
            arguments: vec![
                (
                    "x".to_string(),
                    ExpressionNode::Literal(Box::new(LiteralNode {
                        value: LiteralValue::Integer(0),
                    })),
                ),
                (
                    "y".to_string(),
                    ExpressionNode::Literal(Box::new(LiteralNode {
                        value: LiteralValue::String("shunsock".to_string()),
                    })),
                ),
            ],
        });

        // テストする関数の入力である、Token列, Parserの生成
        // f(x: 0, y: "shunsock");
        let tokens: Vec<Token> = vec![
            Token::new(1, 1, TokenType::Identifier("f".to_string())),
            Token::new(1, 2, TokenType::LeftParen),
            Token::new(1, 1, TokenType::Identifier("x".to_string())),
            Token::new(1, 2, TokenType::Colon),
            Token::new(1, 3, TokenType::IntegerLiteral(0)),
            Token::new(1, 3, TokenType::Comma),
            Token::new(1, 1, TokenType::Identifier("y".to_string())),
            Token::new(1, 2, TokenType::Colon),
            Token::new(1, 3, TokenType::StringLiteral("shunsock".to_string())),
            Token::new(1, 3, TokenType::RightParen),
            Token::new(1, 4, TokenType::Semicolon),
        ];
        let mut parser: Parser = create_parser_with_tokens(tokens);

        // テストしたい関数の出力 (エラーが出ていないことを確認)
        let result: Result<ExpressionNode, ParserError> = parse_identifier_or_call(&mut parser);
        assert!(result.is_ok());

        // テストしたい関数の出力と期待値を比較
        let call_of_function: Box<FunctionCallNode> = match result.clone().unwrap() {
            ExpressionNode::CallOfFunction(call_of_function) => call_of_function,
            _ => panic!("ExpectedNode::CallOfFunction"),
        };
        assert_eq!(call_of_function, expected);
    }

    /// 変数呼び出しが可能か確認するテスト
    /// x;
    #[test]
    fn parse_variable_call() {
        // 生成されるAST Node
        let expected: Box<VariableCallNode> = Box::new(VariableCallNode {
            name: "x".to_string(),
        });

        // テストする関数の入力である、Token列, Parserの生成
        // x;
        let tokens: Vec<Token> = vec![
            Token::new(1, 1, TokenType::Identifier("x".to_string())),
            Token::new(1, 4, TokenType::Semicolon),
        ];
        let mut parser: Parser = create_parser_with_tokens(tokens);

        // テストしたい関数の出力 (エラーが出ていないことを確認)
        let result: Result<ExpressionNode, ParserError> = parse_identifier_or_call(&mut parser);
        assert!(result.is_ok());

        // テストしたい関数の出力と期待値を比較
        let call_of_function: Box<VariableCallNode> = match result.clone().unwrap() {
            ExpressionNode::CallOfVariable(call_of_variable) => call_of_variable,
            _ => panic!("ExpectedNode::CallOfVariable"),
        };
        assert_eq!(call_of_function, expected);
    }

    // 異常系テスト

    /// 引数が複数存在する関数のパースで引数の間にカンマがない時にエラーを出力するか確認するテスト
    /// f(x: 0 y: "shunsock");
    #[test]
    fn parse_function_with_arguments_but_without_comma() {
        // テストする関数の入力である、Token列, Parserの生成
        // f();
        let tokens: Vec<Token> = vec![
            Token::new(1, 1, TokenType::Identifier("f".to_string())),
            Token::new(1, 2, TokenType::LeftParen),
            Token::new(1, 1, TokenType::Identifier("x".to_string())),
            Token::new(1, 2, TokenType::Colon),
            Token::new(1, 3, TokenType::IntegerLiteral(0)),
            // ここにカンマが必要
            Token::new(1, 1, TokenType::Identifier("y".to_string())),
            Token::new(1, 2, TokenType::Colon),
            Token::new(1, 3, TokenType::StringLiteral("shunsock".to_string())),
            Token::new(1, 3, TokenType::RightParen),
            Token::new(1, 4, TokenType::Semicolon),
        ];
        let mut parser: Parser = create_parser_with_tokens(tokens);

        // テストしたい関数の出力 (エラーが出ることを確認)
        let result: Result<ExpressionNode, ParserError> = parse_identifier_or_call(&mut parser);
        assert!(result.is_err());
    }
}
