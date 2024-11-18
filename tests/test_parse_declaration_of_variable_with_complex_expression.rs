use shot::virtual_machine::ast::Statement;
use shot::virtual_machine::parser::declaration_parser::parse_declaration_of_variable::parse_declaration_of_variable;
use shot::virtual_machine::parser::parser_error::ParserError;
use shot::virtual_machine::parser::Parser;
use shot::virtual_machine::token::token_type::TokenType;
use shot::virtual_machine::token::Token;

fn create_parser_with_tokens(tokens: Vec<Token>) -> Parser {
    let mut tokens_with_eof = tokens.clone();
    tokens_with_eof.push(Token::new(1, 1, TokenType::Eof)); // EOFを追加
    Parser::new(tokens_with_eof)
}

// 型が複雑になりすぎるので、okになっているかだけ確認する
// それぞれの詳細はunittestで担保する

/// 二項演算を代入と変数宣言のテスト
/// let num: int = 1 + 0;
#[test]
fn parse_declaration_of_variable_with_binary_operator() {
    // テストする関数の入力である、Token列, Parserの生成
    // name: string = "shunsock";
    // Let token は Let文の処理 で消費されていることに注意
    let tokens: Vec<Token> = vec![
        Token::new(1, 2, TokenType::Identifier("num".to_string())),
        Token::new(1, 3, TokenType::Colon),
        Token::new(1, 4, TokenType::StringType),
        Token::new(1, 5, TokenType::Equal),
        Token::new(1, 6, TokenType::IntegerLiteral(1)),
        Token::new(1, 6, TokenType::Plus),
        Token::new(1, 6, TokenType::IntegerLiteral(1)),
        Token::new(1, 7, TokenType::Semicolon),
    ];
    let mut parser: Parser = create_parser_with_tokens(tokens);

    // テストしたい関数の出力 (エラーが出ていないことを確認)
    let result: Result<Statement, ParserError> = parse_declaration_of_variable(&mut parser);
    assert!(result.is_ok());
}
