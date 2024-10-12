#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // キーワード
    Let,    // let
    As,     // as
    Fn,     // fn
    Return, // return

    // 型
    IntType,    // int
    FloatType,  // float
    StringType, // string
    VoidType,   // void type

    // 識別子
    Identifier(String),

    // リテラル
    IntegerLiteral(i64),   // 整数リテラル
    FloatLiteral(f64),     // 浮動小数点リテラル
    StringLiteral(String), // 文字列リテラル
    NoneLiteral,           // Noneリテラル

    // 記号
    Plus,        // +
    Minus,       // -
    Asterisk,    // *
    Slash,       // /
    Equal,       // =
    Colon,       // :
    Comma,       // ,
    LessThan,    // <
    GreaterThan, // >
    LeftParen,   // (
    RightParen,  // )
    LeftBrace,   // {
    RightBrace,  // }
    Semicolon,   // ;

    // 型キャストのための矢印
    TypeCastArrow, // ->

    // 終了トークン
    Eof, // 終端
}
