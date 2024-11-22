use crate::virtual_machine::ast::{ExpressionNode, Type, TypeCastNode};
use crate::virtual_machine::parser::core::type_token_to_type;
use crate::virtual_machine::parser::parser_error::ParserError;
use crate::virtual_machine::parser::Parser;
use crate::virtual_machine::token::token_type::TokenType;

/// ## Parse a type cast expression.
///
/// ## Syntax
///
/// 1 as int -> string;
pub fn parse_type_cast(
    parser: &mut Parser,
    expr: ExpressionNode,
) -> Result<ExpressionNode, ParserError> {
    // asを確認して読み飛ばす
    parser.check_advance(TokenType::As)?;

    // 型を読み取る
    let from_type: Type = type_token_to_type(parser.peek().token_type.clone())?;
    parser.advance();

    // -> を確認して読み飛ばす
    parser.check_advance(TokenType::TypeCastArrow)?;

    // 型を読み取る
    let to_type: Type = type_token_to_type(parser.peek().token_type.clone())?;
    parser.advance();

    // 式を返す
    Ok(ExpressionNode::TypeCast(Box::new(TypeCastNode {
        from_type,
        to_type,
        expression: Box::new(expr),
    })))
}

#[cfg(test)]
mod tests {
    use crate::virtual_machine::ast::ExpressionNode;
    use crate::virtual_machine::ast::{LiteralNode, LiteralValue, Type, TypeCastNode};
    use crate::virtual_machine::parser::expression_parser::parse_type_cast::parse_type_cast;
    use crate::virtual_machine::parser::parser_error::ParserError;
    use crate::virtual_machine::parser::{Parser, TokenType};
    use crate::virtual_machine::token::Token;

    fn create_parser_with_tokens(tokens: Vec<Token>) -> Parser {
        let mut tokens_with_eof = tokens.clone();
        tokens_with_eof.push(Token::new(1, 1, TokenType::Eof)); // EOFを追加
        Parser::new(tokens_with_eof)
    }

    /// TypeCastのテスト
    /// 1 as int -> string;
    #[test]
    fn parse_cast_from_literal() {
        // 期待される出力
        let expected = ExpressionNode::TypeCast(Box::new(TypeCastNode {
            from_type: Type::Integer,
            to_type: Type::String,
            expression: Box::new(ExpressionNode::Literal(Box::new(LiteralNode {
                value: LiteralValue::Integer(1),
            }))),
        }));

        // テストしたい関数の入力
        let tokens = vec![
            Token::new(1, 1, TokenType::As),
            Token::new(1, 1, TokenType::IntType),
            Token::new(1, 1, TokenType::TypeCastArrow),
            Token::new(1, 1, TokenType::StringType),
            Token::new(1, 1, TokenType::Semicolon),
        ];
        let mut parser: Parser = create_parser_with_tokens(tokens);

        // テストしたい関数の出力 (エラーが出ていないことを確認)
        let result: Result<ExpressionNode, ParserError> = parse_type_cast(
            &mut parser,
            ExpressionNode::Literal(Box::new(LiteralNode {
                value: LiteralValue::Integer(1),
            })),
        );

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }
}
