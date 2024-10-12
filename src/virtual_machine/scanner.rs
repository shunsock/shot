mod scanner_error;

use crate::virtual_machine::token::token_type::TokenType;
use crate::virtual_machine::token::Token;
use scanner_error::ScannerError;

pub(crate) struct Scanner {
    source_code: String,             // ソースコード全体を1つの文字列として扱う
    source_code_vector: Vec<String>, // ソースコードを行ごとに分割したベクター
    current_pos: usize,              // 現在の文字位置
    line: usize,                     // 現在の行番号
    char_pos: usize,                 // 現在の行内の文字位置
}

impl Scanner {
    pub fn new(source_code: String, source_code_vector: Vec<String>) -> Self {
        Scanner {
            source_code,
            source_code_vector,
            current_pos: 0,
            line: 1,
            char_pos: 0,
        }
    }

    pub fn scan(mut self) -> Result<Vec<Token>, ScannerError> {
        let mut tokens: Vec<Token> = Vec::new();

        loop {
            match self.next_token() {
                Ok(token) => {
                    tokens.push(token.clone());
                    if token.token_type == TokenType::Eof {
                        break;
                    }
                }
                Err(e) => return Err(e),
            }
        }

        Ok(tokens)
    }

    // 次のトークンを取得する
    pub fn next_token(&mut self) -> Result<Token, ScannerError> {
        // ソースコード終端のチェック: 終端であればEOFを返して処理を終了
        if self.is_at_end() {
            return Ok(Token::new(self.line, self.char_pos, TokenType::Eof));
        }

        // 空白文字類をスキップするループ
        while self.peek().is_whitespace() {
            self.advance();
            if self.is_at_end() {
                return Ok(Token::new(self.line, self.char_pos, TokenType::Eof));
            }
        }

        // 終端でなければ次のトークンを取得
        let c = self.advance();

        match c {
            // 終端符号
            '\0' => Ok(Token::new(self.line, self.char_pos, TokenType::Eof)),

            // コメントの開始を検出
            '#' => {
                self.skip_comment();
                self.next_token()
            }

            // 識別子の開始 (アルファベットや _ )
            c if c.is_alphabetic() || c == '_' => self.identifier(),

            // 数字
            '0'..='9' => self.number(),

            // 演算子や記号
            '+' => Ok(Token::new(self.line, self.char_pos, TokenType::Plus)),

            '-' => {
                // 矢印演算子の判定
                match self.peek() {
                    '>' => {
                        self.advance();
                        Ok(Token::new(
                            self.line,
                            self.char_pos,
                            TokenType::TypeCastArrow,
                        ))
                    }
                    _ => Ok(Token::new(self.line, self.char_pos, TokenType::Minus)),
                }
            }

            // 文字列リテラル
            '"' => self.string(),

            '*' => Ok(Token::new(self.line, self.char_pos, TokenType::Asterisk)),
            '/' => Ok(Token::new(self.line, self.char_pos, TokenType::Slash)),
            '=' => Ok(Token::new(self.line, self.char_pos, TokenType::Equal)),
            ':' => Ok(Token::new(self.line, self.char_pos, TokenType::Colon)),
            ',' => Ok(Token::new(self.line, self.char_pos, TokenType::Comma)),
            '<' => Ok(Token::new(self.line, self.char_pos, TokenType::LessThan)),
            '>' => Ok(Token::new(self.line, self.char_pos, TokenType::GreaterThan)),
            '{' => Ok(Token::new(self.line, self.char_pos, TokenType::LeftBrace)),
            '}' => Ok(Token::new(self.line, self.char_pos, TokenType::RightBrace)),
            '(' => Ok(Token::new(self.line, self.char_pos, TokenType::LeftParen)),
            ')' => Ok(Token::new(self.line, self.char_pos, TokenType::RightParen)),
            ';' => Ok(Token::new(self.line, self.char_pos, TokenType::Semicolon)),
            _ => Err(ScannerError::UnexpectedToken {
                token: c.to_string(),
                line: self.line,
                char_pos: self.char_pos,
                source_code_line: self.source_code_vector[self.line - 1].clone(),
            }),
        }
    }

    // 現在の文字を進める
    fn advance(&mut self) -> char {
        let c = self.source_code[self.current_pos..]
            .chars()
            .next()
            .unwrap_or('\0');

        self.current_pos += c.len_utf8(); // 文字のバイト数に応じてポインタを進める
        self.char_pos += 1;

        if c == '\n' {
            self.line += 1;
            self.char_pos = 0;
        }

        c
    }

    // 終端判定
    fn is_at_end(&self) -> bool {
        self.current_pos >= self.source_code.len()
    }

    // 次の文字を覗き見る
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.source_code[self.current_pos..]
            .chars()
            .next()
            .unwrap_or('\0')
    }

    // コメントをスキップする関数
    fn skip_comment(&mut self) {
        // シングルラインコメントをスキップ (改行まで)
        while !self.is_at_end() && self.peek() != '\n' {
            self.advance();
        }
    }

    // 識別子のトークン生成
    fn identifier(&mut self) -> Result<Token, ScannerError> {
        let start = self.current_pos - 1;

        // 識別子の終端まで読み進める
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let text = &self.source_code[start..self.current_pos];

        // キーワードの判定
        let token_type = match text {
            "let" => TokenType::Let,
            "as" => TokenType::As,
            "fn" => TokenType::Fn,
            "return" => TokenType::Return,
            "none" => TokenType::NoneLiteral,
            "void" => TokenType::VoidType,
            "int" => TokenType::IntType,
            "float" => TokenType::FloatType,
            "string" => TokenType::StringType,
            _ => TokenType::Identifier(text.to_string()),
        };

        Ok(Token::new(self.line, self.char_pos, token_type))
    }

    /// 数字のトークン生成
    ///
    /// 整数リテラルと浮動小数点リテラルを判定し、トークンを生成する
    ///
    /// 整数リテラル: 1, 42, 100
    /// 浮動小数点リテラル: 3.14, 42.0, 100.0
    fn number(&mut self) -> Result<Token, ScannerError> {
        let start = self.current_pos - 1;
        let mut has_dot = false;

        while self.peek().is_ascii_digit() || (self.peek() == '.' && !has_dot) {
            if self.peek() == '.' {
                has_dot = true;
            }
            self.advance();
        }

        let text = &self.source_code[start..self.current_pos];

        // Check if the number is immediately followed by alphabetic characters
        if self.peek().is_alphabetic() {
            // Consume the invalid alphanumeric sequence
            while self.peek().is_alphanumeric() || self.peek() == '_' {
                self.advance();
            }

            let invalid_text: String = self.source_code[start..self.current_pos].to_string();
            return Err(ScannerError::InvalidCharacterInNumberLiteral {
                number_literal: invalid_text,
                line: self.line,
                char_pos: start,
                source_code_line: self.source_code_vector[self.line - 1].clone(),
            });
        }

        if has_dot {
            let parsed_text = text.parse::<f64>();
            return match parsed_text {
                Ok(float_value) => Ok(Token::new(
                    self.line,
                    self.char_pos,
                    TokenType::FloatLiteral(float_value),
                )),
                Err(text) => Err(ScannerError::InvalidFloatLiteralFound {
                    number: text.to_string(),
                    line: self.line,
                    char_pos: start,
                    source_code_line: self.source_code_vector[self.line - 1].clone(),
                }),
            };
        }
        match text.parse::<i64>() {
            Ok(int_value) => Ok(Token::new(
                self.line,
                self.char_pos,
                TokenType::IntegerLiteral(int_value),
            )),
            Err(text) => Err(ScannerError::InvalidIntegerLiteralFound {
                number: text.to_string(),
                line: self.line,
                char_pos: start,
                source_code_line: self.source_code_vector[self.line - 1].clone(),
            }),
        }
    }

    // 文字列リテラルのトークン生成
    fn string(&mut self) -> Result<Token, ScannerError> {
        let mut value = String::new();
        let start = self.current_pos - 1;

        while self.peek() != '"' && !self.is_at_end() {
            value.push(self.advance());
        }

        if self.is_at_end() {
            return Err(ScannerError::UnterminatedString {
                string: value,
                line: self.line,
                char_pos: start,
                source_code_line: self.source_code_vector[self.line - 1].clone(),
            });
        }

        // 終わりの '"' を消費
        self.advance();

        Ok(Token::new(
            self.line,
            self.char_pos,
            TokenType::StringLiteral(value),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 単純な識別子を認識可能か確認するテスト
    /// Scannerが単純な識別子 "foo" を `Token::Identifier` に正しくトークン化するかをテストします。
    #[test]
    fn test_simple_identifier() {
        let source = "foo".to_string();
        let source_vector = vec!["foo".to_string()];

        let mut scanner = Scanner::new(source, source_vector);

        let token = scanner.next_token().unwrap();

        // トークンが "foo" のIdentifierであることをアサート
        assert_eq!(token.token_type, TokenType::Identifier("foo".to_string()));
    }

    /// 空白文字をスキップ可能か確認するテスト
    /// Scannerが存在しないトークン "!" を含むソースコードを検出し、エラーを返すかをテストします。
    #[test]
    fn invalid_token_found() {
        // 無効なトークン "!" を含むソースコードを作成
        let source = "!".to_string();
        let source_vector = vec!["!".to_string()];

        let mut scanner = Scanner::new(source, source_vector);

        let result = scanner.next_token();

        // 結果がScannerError::UnexpectedToken型のエラーであることをアサートします。
        assert!(matches!(result, Err(ScannerError::UnexpectedToken { .. })));
    }

    /// キーワードを認識可能か確認するテスト
    /// Scannerがキーワード "let" を正しく認識し、`Token::Let` にトークン化するかをテストします。
    #[test]
    fn test_keyword_let() {
        let source = "let".to_string();
        let source_vector = vec!["let".to_string()];

        let mut scanner = Scanner::new(source, source_vector);

        let token = scanner.next_token().unwrap();

        // トークンが Token::Let であることをアサート
        assert_eq!(token.token_type, TokenType::Let);
    }

    /// 数字を含む識別子を認識可能か確認するテスト
    /// Scannerが数字 "variable1" を `Token::Identifier` に正しくトークン化するかをテストします。
    #[test]
    fn test_identifier_with_numbers() {
        let source = "variable1".to_string();
        let source_vector = vec!["variable1".to_string()];

        let mut scanner = Scanner::new(source, source_vector);

        let token = scanner.next_token().unwrap();

        // トークンが "variable1" のIdentifierであることをアサート
        assert_eq!(
            token.token_type,
            TokenType::Identifier("variable1".to_string())
        );
    }

    /// 整数リテラルを認識可能か確認するテスト
    /// 整数リテラル "123" を `Token::IntegerLiteral` に正しくトークン化するかをテストします。
    #[test]
    fn test_integer_literal() {
        let source = "123".to_string();
        let source_vector = vec!["123".to_string()];

        let mut scanner = Scanner::new(source, source_vector);

        let token = scanner.next_token().unwrap();

        // トークンが整数リテラル "123" であることをアサート
        assert_eq!(token.token_type, TokenType::IntegerLiteral(123));
    }

    /// 浮動小数点リテラルを認識可能か確認するテスト
    /// 浮動小数点リテラル "3.14" を `Token::FloatLiteral` に正しくトークン化するかをテストします。
    #[test]
    fn test_float_literal() {
        let source = "3.14".to_string();
        let source_vector = vec!["3.14".to_string()];

        let mut scanner = Scanner::new(source, source_vector);

        let token = scanner.next_token().unwrap();

        // トークンが浮動小数点リテラル "3.14" であることをアサート
        assert_eq!(token.token_type, TokenType::FloatLiteral(3.14));
    }

    /// 無効な数字を認識可能か確認するテスト
    /// 無効な数字 "123abc" を与えたときに、Scannerが `InvalidNumber` エラーを返すかをテストします。
    #[test]
    fn test_invalid_number() {
        // 無効な数字 "123abc" を含むソースコードを作成
        let source = "123abc".to_string();
        let source_vector = vec!["123abc".to_string()];

        let mut scanner = Scanner::new(source, source_vector);

        let result = scanner.next_token();

        // 結果がScannerError::InvalidNumber型のエラーであることをアサートします。
        assert!(matches!(
            result,
            Err(ScannerError::InvalidCharacterInNumberLiteral { .. })
        ));
    }

    /// 文字列リテラルを認識可能か確認するテスト
    /// 文字列リテラル "\"hello world\"" を `Token::StringLiteral` に正しくトークン化するかをテストします。
    #[test]
    fn test_string_literal() {
        let source = "\"hello world\"".to_string();
        let source_vector = vec!["\"hello world\"".to_string()];

        let mut scanner = Scanner::new(source, source_vector);

        let token = scanner.next_token().unwrap();

        // トークンが "hello world" のStringLiteralであることをアサート
        assert_eq!(
            token.token_type,
            TokenType::StringLiteral("hello world".to_string())
        );
    }

    /// 終端されていない文字列リテラルを認識可能か確認するテスト
    /// 終端されていない文字列リテラル "\"hello world" を与えたときに、Scannerが `UnterminatedString` エラーを返すかをテストします。
    #[test]
    fn test_unterminated_string() {
        let source = "\"hello world".to_string();
        let source_vector = vec!["\"hello world".to_string()];

        let mut scanner = Scanner::new(source, source_vector);

        let result = scanner.next_token();

        // 結果がScannerError::UnterminatedString型のエラーであることをアサートします。
        assert!(matches!(
            result,
            Err(ScannerError::UnterminatedString { .. })
        ));
    }

    /// 空の文字列リテラルを認識可能か確認するテスト
    /// 空の文字列リテラル "\"\"" を `Token::StringLiteral` に正しくトークン化するかをテストします。
    #[test]
    fn test_empty_string_literal() {
        let source = "\"\"".to_string();
        let source_vector = vec!["\"\"".to_string()];

        let mut scanner = Scanner::new(source, source_vector);

        let token = scanner.next_token().unwrap();

        // トークンが空のStringLiteralであることをアサート
        assert_eq!(token.token_type, TokenType::StringLiteral("".to_string()));
    }

    /// シンプルなコメントが正しくスキップされるか確認するテスト
    #[test]
    fn test_simple_comment() {
        let source = "# This is a comment\nlet".to_string();
        let source_vector = vec!["# This is a comment".to_string(), "let".to_string()];

        let mut scanner = Scanner::new(source, source_vector);

        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Let);
    }

    /// コメント行の後にトークンが続く場合でも正しく処理できるか確認するテスト
    #[test]
    fn test_comment_followed_by_code() {
        let source = "# comment\nlet x = 42;".to_string();
        let source_vector = vec!["# comment".to_string(), "let x = 42;".to_string()];

        let mut scanner = Scanner::new(source, source_vector);

        // "let" トークンのチェック
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Let);

        // "x" 識別子のチェック
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Identifier("x".to_string()));

        // "=" トークンのチェック
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Equal);

        // "42" 整数リテラルのチェック
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::IntegerLiteral(42));

        // ";" トークンのチェック
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Semicolon);
    }

    /// コメントのみのソースコードを処理できるか確認するテスト
    #[test]
    fn test_only_comment() {
        let source = "# This is a comment".to_string();
        let source_vector = vec!["# This is a comment".to_string()];

        let mut scanner = Scanner::new(source, source_vector);

        // EOFトークンが生成されることを確認
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Eof);
    }

    /// 複数のコメント行を正しくスキップできるか確認するテスト
    #[test]
    fn test_multiple_comments() {
        let source = "# First comment\n# Second comment\nlet".to_string();
        let source_vector = vec![
            "# First comment".to_string(),
            "# Second comment".to_string(),
            "let".to_string(),
        ];

        let mut scanner = Scanner::new(source, source_vector);

        // "let" トークンが正しく取得されることを確認
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Let);
    }

    /// 空白とコメントの組み合わせが正しく処理されるか確認するテスト
    #[test]
    fn test_spaces_and_comments() {
        let source = "   # comment\n   let".to_string();
        let source_vector = vec!["   # comment".to_string(), "   let".to_string()];

        let mut scanner = Scanner::new(source, source_vector);

        // "let" トークンが正しく取得されることを確認
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Let);
    }

    /// コメント行とコードが同じ行にある場合に、コードが正しく処理されるか確認するテスト
    #[test]
    fn test_inline_comment() {
        let source = "let x = 10; # inline comment".to_string();
        let source_vector = vec!["let x = 10; # inline comment".to_string()];

        let mut scanner = Scanner::new(source, source_vector);

        // "let" トークンのチェック
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Let);

        // "x" 識別子のチェック
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Identifier("x".to_string()));

        // "=" トークンのチェック
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Equal);

        // "10" 整数リテラルのチェック
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::IntegerLiteral(10));

        // ";" トークンのチェック
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Semicolon);

        // コメントがあるが、正しくスキップされるかを確認
        let token = scanner.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Eof);
    }
}
