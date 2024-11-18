use crate::virtual_machine::ast::VariableDeclarationNode;
use crate::virtual_machine::ast::{ExpressionNode, Statement, Type};
use crate::virtual_machine::parser::core::type_token_to_type;
use crate::virtual_machine::parser::expression_parser::parse_expression;
use crate::virtual_machine::parser::Parser;
use crate::virtual_machine::parser::ParserError;
use crate::virtual_machine::token::token_type::TokenType;

pub(crate) fn parse_declaration_of_variable(parser: &mut Parser) -> Result<Statement, ParserError> {
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
    parser.check_advance(TokenType::Colon)?;

    // 型を読み取る
    let type_token: TokenType = parser.peek().token_type.clone();
    let variable_type: Type = type_token_to_type(type_token)?;
    parser.advance();

    // イコールを読み飛ばす
    parser.check_advance(TokenType::Equal)?;

    // 式をパース
    let expr: ExpressionNode = parse_expression(parser)?;

    // セミコロンを読み飛ばす
    parser.check_advance(TokenType::Semicolon)?;

    Ok(Statement::DeclarationOfVariable(Box::new(
        VariableDeclarationNode {
            name,
            var_type: variable_type,
            value: Box::new(expr),
        },
    )))
}

#[cfg(test)]
mod tests {
    use crate::virtual_machine::ast::{ExpressionNode, LiteralNode, LiteralValue, Statement, Type, VariableDeclarationNode};
    use crate::virtual_machine::parser::{Parser, TokenType};
    use crate::virtual_machine::parser::declaration_parser::parse_declaration_of_variable::parse_declaration_of_variable;
    use crate::virtual_machine::parser::parser_error::ParserError;
    use crate::virtual_machine::token::Token;

    fn create_parser_with_tokens(tokens: Vec<Token>) -> Parser {
        let mut tokens_with_eof = tokens.clone();
        tokens_with_eof.push(Token::new(1, 1, TokenType::Eof)); // EOFを追加
        Parser::new(tokens_with_eof)
    }

    /// string型の変数の変数宣言のテスト
    /// let name: string = "shunsock";
    #[test]
    fn parses_string_variable_declaration() {
        // 生成されるAST Node
        let expected = Box::new(VariableDeclarationNode {
            name: "name".to_string(),
            var_type: Type::String,
            value: Box::new(
                ExpressionNode::Literal(
                    Box::new(LiteralNode {
                        value: LiteralValue::String("shunsock".to_string()),
                    })
                )
            ),
        });

        // テストする関数の入力である、Token列, Parserの生成
        // name: string = "shunsock";
        // Let token は Let文の処理 で消費されていることに注意
        let tokens: Vec<Token> = vec![
            Token::new(1, 2, TokenType::Identifier("name".to_string())),
            Token::new(1, 3, TokenType::Colon),
            Token::new(1, 4, TokenType::StringType),
            Token::new(1, 5, TokenType::Equal),
            Token::new(1, 6, TokenType::StringLiteral("shunsock".to_string())),
            Token::new(1, 7, TokenType::Semicolon),
        ];
        let mut parser: Parser = create_parser_with_tokens(tokens);

        // テストしたい関数の出力 (エラーが出ていないことを確認)
        let result: Result<Statement, ParserError> = parse_declaration_of_variable(&mut parser);
        assert!(result.is_ok());

        // テストしたい関数の出力と期待値を比較
        let variable_declaration_node: Box<VariableDeclarationNode> = match result.unwrap() {
            Statement::DeclarationOfVariable(node) => node,
            _ => panic!("Expected a DeclarationOfVariable"),
        };
        assert_eq!(variable_declaration_node, expected);
    }

    /// Integer型の変数の変数宣言のテスト
    /// let num: int = 0;
    #[test]
    fn parses_integer_variable_declaration() {
        // 生成されるAST Node
        let expected = Box::new(VariableDeclarationNode {
            name: "num".to_string(),
            var_type: Type::Integer,
            value: Box::new(
                ExpressionNode::Literal(
                    Box::new(LiteralNode {
                        value: LiteralValue::Integer(0),
                    })
                )
            ),
        });

        // テストする関数の入力である、Token列, Parserの生成
        // num: int = 0;
        // Let token は Let文の処理 で消費されていることに注意
        let tokens: Vec<Token> = vec![
            Token::new(1, 2, TokenType::Identifier("num".to_string())),
            Token::new(1, 3, TokenType::Colon),
            Token::new(1, 4, TokenType::IntType),
            Token::new(1, 5, TokenType::Equal),
            Token::new(1, 6, TokenType::IntegerLiteral(0)),
            Token::new(1, 7, TokenType::Semicolon),
        ];
        let mut parser: Parser = create_parser_with_tokens(tokens);

        // テストしたい関数の出力 (エラーが出ていないことを確認)
        let result: Result<Statement, ParserError> = parse_declaration_of_variable(&mut parser);
        assert!(result.is_ok());

        // テストしたい関数の出力と期待値を比較
        let variable_declaration_node: Box<VariableDeclarationNode> = match result.unwrap() {
            Statement::DeclarationOfVariable(node) => node,
            _ => panic!("Expected a DeclarationOfVariable"),
        };
        assert_eq!(variable_declaration_node, expected);
    }

    /// Float型の変数の変数宣言のテスト
    /// let num: float = 0.0;
    #[test]
    fn parses_float_variable_declaration() {
        // 生成されるAST Node
        let expected = Box::new(VariableDeclarationNode {
            name: "num".to_string(),
            var_type: Type::Float,
            value: Box::new(
                ExpressionNode::Literal(
                    Box::new(LiteralNode {
                        value: LiteralValue::Float(0.0),
                    })
                )
            ),
        });

        // テストする関数の入力である、Token列, Parserの生成
        // num: float = 0;
        // Let token は Let文の処理 で消費されていることに注意
        let tokens: Vec<Token> = vec![
            Token::new(1, 2, TokenType::Identifier("num".to_string())),
            Token::new(1, 3, TokenType::Colon),
            Token::new(1, 4, TokenType::FloatType),
            Token::new(1, 5, TokenType::Equal),
            Token::new(1, 6, TokenType::FloatLiteral(0.0)),
            Token::new(1, 7, TokenType::Semicolon),
        ];
        let mut parser: Parser = create_parser_with_tokens(tokens);

        // テストしたい関数の出力 (エラーが出ていないことを確認)
        let result: Result<Statement, ParserError> = parse_declaration_of_variable(&mut parser);
        assert!(result.is_ok());

        // テストしたい関数の出力と期待値を比較
        let variable_declaration_node: Box<VariableDeclarationNode> = match result.unwrap() {
            Statement::DeclarationOfVariable(node) => node,
            _ => panic!("Expected a DeclarationOfVariable"),
        };
        assert_eq!(variable_declaration_node, expected);
    }


    /// void型の変数の変数宣言のテスト
    /// let value: void = none;
    #[test]
    fn parses_void_variable_declaration() {
        // 生成されるAST Node
        let expected = Box::new(VariableDeclarationNode {
            name: "value".to_string(),
            var_type: Type::Void,
            value: Box::new(
                ExpressionNode::Literal(
                    Box::new(LiteralNode {
                        value: LiteralValue::None,
                    })
                )
            ),
        });

        // テストする関数の入力である、Token列, Parserの生成
        // value: void = none;
        // Let token は Let文の処理 で消費されていることに注意
        let tokens: Vec<Token> = vec![
            Token::new(1, 2, TokenType::Identifier("value".to_string())),
            Token::new(1, 3, TokenType::Colon),
            Token::new(1, 4, TokenType::VoidType),
            Token::new(1, 5, TokenType::Equal),
            Token::new(1, 6, TokenType::NoneLiteral),
            Token::new(1, 7, TokenType::Semicolon),
        ];
        let mut parser: Parser = create_parser_with_tokens(tokens);

        // テストしたい関数の出力 (エラーが出ていないことを確認)
        let result: Result<Statement, ParserError> = parse_declaration_of_variable(&mut parser);
        assert!(result.is_ok());

        // テストしたい関数の出力と期待値を比較
        let variable_declaration_node: Box<VariableDeclarationNode> = match result.unwrap() {
            Statement::DeclarationOfVariable(node) => node,
            _ => panic!("Expected a DeclarationOfVariable"),
        };
        assert_eq!(variable_declaration_node, expected);
    }
}
