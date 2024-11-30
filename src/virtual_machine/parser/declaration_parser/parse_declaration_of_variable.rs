use crate::virtual_machine::ast::VariableDeclarationNode;
use crate::virtual_machine::ast::{ExpressionNode, Statement, Type};
use crate::virtual_machine::parser::core::get_type_from_current_token;
use crate::virtual_machine::parser::expression_parser::parse_expression;
use crate::virtual_machine::parser::Parser;
use crate::virtual_machine::parser::ParserError;
use crate::virtual_machine::token::token_type::TokenType;

/// 変数宣言文をパースする
///
/// # Returns
/// - `Result<Statement, ParserError>`: パース結果
///   - `Statement`: パース結果のASTノード
///   - `ParserError`: エラー情報
///
/// # Syntax
/// 変数宣言文は、プログラムで用いる変数を宣言する文の一つ。
///
/// - 変数宣言文`variable_declaration = "let" identifier ":" type "=" expression ";"`
///
/// # Examples
/// - string型の変数nameを宣言: `let name: string = "shunsock";`
/// - int型の変数numを宣言: `let num: int = 0;`
///
/// 変数宣言時に代入する値は、式によって定められる。
///
/// - BinaryOperationが式である事例: `let num: int = 1 + 2;`
pub fn parse_declaration_of_variable(parser: &mut Parser) -> Result<Statement, ParserError> {
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
    let variable_type: Type = get_type_from_current_token(parser)?;
    parser.advance();

    // イコールを読み飛ばす
    parser.check_advance(TokenType::Equal)?;

    // 式をパース
    let expr: ExpressionNode = parse_expression(parser)?;

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
    use crate::virtual_machine::ast::{
        ExpressionNode, LiteralNode, LiteralValue, Statement, Type, VariableDeclarationNode,
    };
    use crate::virtual_machine::parser::core::create_parser_with_tokens;
    use crate::virtual_machine::parser::declaration_parser::parse_declaration_of_variable::parse_declaration_of_variable;
    use crate::virtual_machine::parser::parser_error::ParserError;
    use crate::virtual_machine::parser::{Parser, TokenType};
    use crate::virtual_machine::token::Token;

    /// string型の変数の変数宣言のテスト
    /// let name: string = "shunsock";
    #[test]
    fn parses_string_variable_declaration() {
        // 生成されるAST Node
        let expected = Box::new(VariableDeclarationNode {
            name: "name".to_string(),
            var_type: Type::String,
            value: Box::new(ExpressionNode::Literal(Box::new(LiteralNode {
                value: LiteralValue::String("shunsock".to_string()),
            }))),
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

    // 正常系テスト

    /// Integer型の変数の変数宣言のテスト
    /// let num: int = 0;
    #[test]
    fn parses_integer_variable_declaration() {
        // 生成されるAST Node
        let expected = Box::new(VariableDeclarationNode {
            name: "num".to_string(),
            var_type: Type::Integer,
            value: Box::new(ExpressionNode::Literal(Box::new(LiteralNode {
                value: LiteralValue::Integer(0),
            }))),
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
            value: Box::new(ExpressionNode::Literal(Box::new(LiteralNode {
                value: LiteralValue::Float(0.0),
            }))),
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
            value: Box::new(ExpressionNode::Literal(Box::new(LiteralNode {
                value: LiteralValue::None,
            }))),
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

    // 異常系テスト

    /// colonがない場合にエラーを出力するか確認するテスト
    /// let name string = "shunsock";  # error
    #[test]
    fn raise_error_without_colon() {
        // テストする関数の入力である、Token列, Parserの生成
        // name: string = "shunsock";
        // Let token は Let文の処理 で消費されていることに注意
        let tokens: Vec<Token> = vec![
            Token::new(1, 1, TokenType::Identifier("name".to_string())),
            Token::new(1, 2, TokenType::StringType),
            Token::new(1, 3, TokenType::Equal),
            Token::new(1, 4, TokenType::StringLiteral("shunsock".to_string())),
            Token::new(1, 5, TokenType::Semicolon),
        ];
        let mut parser: Parser = create_parser_with_tokens(tokens);

        // テストしたい関数の出力 (エラーになることを確認)
        let result: Result<Statement, ParserError> = parse_declaration_of_variable(&mut parser);
        assert!(result.is_err());
    }

    /// type annotationがない場合にエラーを出力するか確認するテスト
    /// let name: = "shunsock";  # error
    #[test]
    fn raise_error_without_type_annotation() {
        // テストする関数の入力である、Token列, Parserの生成
        // name: string = "shunsock";
        // Let token は Let文の処理 で消費されていることに注意
        let tokens: Vec<Token> = vec![
            Token::new(1, 1, TokenType::Identifier("name".to_string())),
            Token::new(1, 2, TokenType::Colon),
            Token::new(1, 3, TokenType::Equal),
            Token::new(1, 4, TokenType::StringLiteral("shunsock".to_string())),
            Token::new(1, 5, TokenType::Semicolon),
        ];
        let mut parser: Parser = create_parser_with_tokens(tokens);

        // テストしたい関数の出力 (エラーになることを確認)
        let result: Result<Statement, ParserError> = parse_declaration_of_variable(&mut parser);
        assert!(result.is_err());
    }

    /// equalがない場合にエラーを出力するか確認するテスト
    /// let name: string "shunsock";  # error
    #[test]
    fn raise_error_without_equal() {
        // テストする関数の入力である、Token列, Parserの生成
        // name: string "shunsock";
        // Let token は Let文の処理 で消費されていることに注意
        let tokens: Vec<Token> = vec![
            Token::new(1, 1, TokenType::Identifier("name".to_string())),
            Token::new(1, 2, TokenType::Colon),
            Token::new(1, 3, TokenType::StringType),
            Token::new(1, 4, TokenType::StringLiteral("shunsock".to_string())),
            Token::new(1, 5, TokenType::Semicolon),
        ];
        let mut parser: Parser = create_parser_with_tokens(tokens);

        // テストしたい関数の出力 (エラーになることを確認)
        let result: Result<Statement, ParserError> = parse_declaration_of_variable(&mut parser);
        assert!(result.is_err());
    }

    /// expressionがない場合にエラーを出力するか確認するテスト
    /// let name: string = ;  # error
    #[test]
    fn raise_error_without_expression() {
        // テストする関数の入力である、Token列, Parserの生成
        // name: string = ;
        // Let token は Let文の処理 で消費されていることに注意
        let tokens: Vec<Token> = vec![
            Token::new(1, 1, TokenType::Identifier("name".to_string())),
            Token::new(1, 2, TokenType::Colon),
            Token::new(1, 3, TokenType::StringType),
            Token::new(1, 4, TokenType::Equal),
            Token::new(1, 5, TokenType::Semicolon),
        ];
        let mut parser: Parser = create_parser_with_tokens(tokens);

        // テストしたい関数の出力 (エラーになることを確認)
        let result: Result<Statement, ParserError> = parse_declaration_of_variable(&mut parser);
        assert!(result.is_err());
    }
}
