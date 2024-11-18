use crate::virtual_machine::ast::{ExpressionNode, FunctionCallNode, VariableCallNode};
use crate::virtual_machine::parser::expression_parser::parse_expression;
use crate::virtual_machine::parser::parser_error::ParserError;
use crate::virtual_machine::parser::Parser;
use crate::virtual_machine::token::token_type::TokenType;

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

    // 次のトークンが左括弧なら関数呼び出し
    match parser.peek().token_type {
        TokenType::LeftParen => {
            // LeftParenを読み飛ばす
            parser.advance();

            // 引数を読み込む
            let mut args: Vec<ExpressionNode> = Vec::new();
            while !parser.check(TokenType::RightParen) {
                match parser.peek().token_type.clone() {
                    TokenType::Comma => {
                        // カンマがあれば読み飛ばす
                        parser.advance();
                    }
                    _ => {
                        // 引数を読み込む
                        args.push(parse_expression(parser)?);
                    }
                };
            }

            // 右括弧があることを確認して読み飛ばす
            parser.check_advance(TokenType::RightParen)?;

            Ok(ExpressionNode::CallOfFunction(Box::new(FunctionCallNode {
                name: name.clone(),
                arguments: args,
            })))
        }
        _ => {
            // 変数呼び出し
            Ok(ExpressionNode::CallOfVariable(Box::new(VariableCallNode {
                name: name.clone(),
            })))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::virtual_machine::ast::{LiteralNode, LiteralValue};
    use crate::virtual_machine::parser::Parser;
    use crate::virtual_machine::token::{token_type::TokenType, Token};

    fn create_parser_with_tokens(tokens: Vec<Token>) -> Parser {
        let mut tokens_with_eof = tokens.clone();
        tokens_with_eof.push(Token::new(1, 1, TokenType::Eof)); // EOFを追加
        Parser::new(tokens_with_eof)
    }

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

    /// 引数が存在する関数のパースが可能か確認するテスト
    /// f(0);
    #[test]
    fn parse_function_with_an_argument() {
        // 生成されるAST Node
        let expected: Box<FunctionCallNode> = Box::new(FunctionCallNode {
            name: "f".to_string(),
            arguments: vec![ExpressionNode::Literal(Box::new(LiteralNode {
                value: LiteralValue::Integer(0),
            }))],
        });

        // テストする関数の入力である、Token列, Parserの生成
        // f();
        let tokens: Vec<Token> = vec![
            Token::new(1, 1, TokenType::Identifier("f".to_string())),
            Token::new(1, 2, TokenType::LeftParen),
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
}
