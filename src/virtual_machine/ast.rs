// ---------------------------------------------------------------------
// Abstract Syntax Tree
// ---------------------------------------------------------------------
#[derive(Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub struct AST {
    pub(crate) statements: Vec<(usize, Statement)>,
}

impl AST {
    pub(crate) fn new() -> Self {
        Self { statements: vec![] }
    }

    pub(crate) fn push_statement(&mut self, line: usize, statement: Statement) {
        self.statements.push((line, statement))
    }
}

// ---------------------------------------------------------------------
// Statements
// ---------------------------------------------------------------------

/// ## Statementノード
///
/// StatementNode は、プログラムの文を表すノードです。
/// 例えば、関数宣言、変数宣言、return文などが StatementNode に含まれます。
///
/// ```shot
/// let f: fn  = add(a: int, b: int): int { };
/// let a: int = 1;
/// return a;
/// ```
///
/// Statementは必ず文末にセミコロンがつきます。
/// 例えば、下記のようなコードはエラーとなります。
///
/// ```shot
/// let a: int = 1
/// ```
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Expression(ExpressionNode),
    DeclarationOfFunction(Box<FunctionDeclarationNode>), // 関数宣言
    DeclarationOfVariable(Box<VariableDeclarationNode>), // 変数宣言
    Return(Box<ExpressionNode>),                         // return文
}

/// ## 変数宣言ノード
///
/// 変数宣言ノードは、変数の宣言を表すノードです。
/// 例えば、`let a: int = 1;` のような変数宣言が VariableDeclarationNode に含まれます。
///
/// 全ての変数宣言はletで始まります。
/// 例えば、`const a: int = 1;` のようなconstで始まる変数宣言はエラーとなります。
/// また、letがない場合は、変数として認識されません。
///
/// 変数宣言ノードは、変数名、型、初期化式を持ちます。
/// 例えば、`let a: int = 1;` の場合、変数名は `a`、型は `int`、初期化式は `1` になります。
/// 初期化式はExpressionです。`let a: int = 1 + 2;` の場合、初期化式は `1 + 2` になります。
#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclarationNode {
    pub name: String,               // 変数名
    pub var_type: Type,             // 型 (例: "int")
    pub value: Box<ExpressionNode>, // 初期化式 (リテラルや式)
}

/// 関数宣言ノード
///
/// 関数宣言ノードは、関数の宣言を表すノードです。
///
/// 関数宣言ノードは、関数名、パラメータリスト、戻り値の型、関数の本体を持ちます。
/// 例えば、`let add: fn = (a: int, b: int): int { return a + b; };` の場合、
/// 関数名は `add`、パラメータリストは `[(a, int), (b, int)]`、戻り値の型は `int`、関数の本体は `return a + b;` になります。
/// また、 `let hello: fn = (): void { print("Hello, World!"); return none };` の場合、
/// 関数名は `hello`、パラメータリストは `[]`、戻り値の型は `void`、関数の本体は `print("Hello, World!");` になります。
///
/// 関数宣言ノードは、関数の本体が複数の文 (Statement) から構成されるため、StatementNode のリストを持ちます。
/// 例えば、
/// ```shot
/// let add: fn = (a: int, b: int): int {
///    let c: int = a + b;
///   return c;
/// };
/// ```
/// の場合、関数の本体は `let c: int = a + b;` と `return c;` の2つの文から構成されます。
///
/// 関数の本体は必ず Return Statement で終わります。
/// 例えば、下記のようなコードはエラーとなります。
///
/// ```shot
/// let add: fn = (a: int, b: int): int {
///   a + b;
/// };
///
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclarationNode {
    pub name: String,                // 関数名
    pub params: Vec<(String, Type)>, // パラメータのリスト (名前と型)
    pub return_type: Type,           // 戻り値の型
    pub body: Vec<Statement>,        // 関数の本体 (ステートメントのリスト)
}

/// ## Returnノード
///
/// Returnノードは、関数の戻り値を表すノードです。
/// 例えば、`return a + b;` のような戻り値が ReturnNode に含まれます。
///
/// Returnノードは常に関数の最後に現れます。
/// 例えば、下記のようなコードは正しいコードです。
///
/// ```shot
/// let add: fn = (a: int, b: int): int {
///   return a + b;
/// };
/// ```
///
/// 例えば、下記のようなコードはエラーとなります。
///
/// ```shot
/// let add: fn = (a: int, b: int): int {
///  return a + b;
///  a + b;
/// };
/// ```
///
/// 関数の中に無い場合はエラーとなります。
/// 例えば、下記のようなコードはエラーとなります。
///
/// ```shot
/// let add: fn = (a: int, b: int): int { return a + b; };
/// return 0;
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct ReturnNode {
    pub expression: Box<ExpressionNode>, // 戻り値として返す式
}

// ---------------------------------------------------------------------
// Expressions
// ---------------------------------------------------------------------

/// ## Expressionノード
///
/// ExpressionNode は、プログラムの式を表すノードです。
/// 式は必ず値を返します。値が存在しない場合は、Type: Void, Value: None を返します。
///
/// 例えば、下記のような式が ExpressionNode に含まれます。
///
/// ```shot
/// 1 + 2;              // 1 + 2 の部分
/// add(1, 2);          // add(1, 2) の部分
/// a;                  // a の部分
/// 1;                  // 1 の部分
/// 1 as int -> float;  // 1 as int -> float の部分
/// ```
///
/// 注意点として、式は文の一部として使われることがあります。
/// 例えば、`let a: int = 1 + 2;` の場合、`1 + 2` は式ですが、`let a: int = 1 + 2;` は文です。
#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionNode {
    BinaryOperation(Box<BinaryOperationNode>), // 二項演算
    CallOfFunction(Box<FunctionCallNode>),     // 関数呼び出し
    CallOfVariable(Box<VariableCallNode>),     // 識別子
    Literal(Box<LiteralNode>),                 // リテラル
    #[allow(dead_code)]
    TypeCast(Box<TypeCastNode>), // 型キャスト
}

/// ## 二項演算ノード
///
/// 二項演算ノードは、2つの式を取る演算を表すノードです。
///
/// 二項演算ノードは、左辺の式、演算子、右辺の式を持ちます。
/// 例えば、`a + b` の場合、左辺の式は `a`、演算子は `+`、右辺の式は `b` になります。
///
/// 二項演算子は、加算、減算、乗算、除算などがあります。
///
/// ```shot
/// 1 + 2
/// 3 - 4
/// 5 * 6
/// 7 / 8
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryOperationNode {
    pub left: Box<ExpressionNode>,  // 左辺の式
    pub operator: BinaryOperator,   // 演算子
    pub right: Box<ExpressionNode>, // 右辺の式
}

/// ## 関数呼び出しノード
///
/// 関数呼び出しノードは、関数の呼び出しを表すノードです。
///
/// 関数呼び出しノードは、関数名、引数リストを持ちます。
/// 例えば、`add(1, 2)` の場合、関数名は `add`、引数リストは `[1, 2]` になります。
///
/// 引数リストは、関数に渡す引数のリストです。
/// 例えば、`add(1, 2)` の場合、引数リストは `[1, 2]` になります。
/// 引数リストは、式のリストです。`add(1 + 2, 3 * 4)` の場合、引数リストは `[1 + 2, 3 * 4]` になります。
/// つまり、下記のようなコードは正しいコードです。
///
/// ```shot
/// add(1, 2);
/// add((1 + 2) as int -> float, 3 * 4);
/// ```
///
/// また、引数が無い場合、引数リストは空リストになります。
/// 例えば、`add()` の場合、引数リストは `[]` になります。
///
/// 一方で、下記のようなコードはエラーとなります。
///
/// ```shot
/// add(1, let n: int = 2;);  // let n: int = 2; はStatementであり、式ではないためエラー
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCallNode {
    pub name: String,                   // 呼び出される関数名
    pub arguments: Vec<ExpressionNode>, // 関数に渡す引数リスト
}

/// ## 変数呼び出しノード
///
/// 変数呼び出しノードは、変数の呼び出しを表すノードです。
///
/// 変数呼び出しノードは、変数名を持ちます。
/// 例えば、`a` の場合、変数名は `a` になります。
///
/// 変数名の解決は、ASTを解析する際に行われます。
/// 例えば、下記のコードは、Parserではエラーにならず、Resolverでエラーとなります。
///
/// ```shot
/// let a: int = 1;
/// print(not_a);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct VariableCallNode {
    pub name: String, // 変数名
}

// リテラルノード (数値や文字列などのリテラル値)
#[derive(Debug, Clone, PartialEq)]
pub struct LiteralNode {
    pub value: LiteralValue,
}

/// ## 型キャストノード
///
/// 型キャストノードは、式 (Expression)を特定の型にキャストするノードです。
/// 例えば、`1 as int -> float` のようなキャストが TypeCastNode に含まれます。
/// この場合、`1` は整数型 `int` から、浮動小数点型 `float` にキャストされます。
///
/// 型キャストの構文は、`式 as 型1 -> 型2` です。
/// 型キャストは式なので、値を返します。具体的には、キャスト先の型にキャストされた値を返します。
/// 例えば、`1 as int -> float` の場合、`1` は `float` 型にキャストされた値 `1.0` を返します。
#[derive(Debug, Clone, PartialEq)]
pub struct TypeCastNode {
    pub from_type: Type,                 // キャスト元の型
    pub to_type: Type,                   // キャスト先の型
    pub expression: Box<ExpressionNode>, // キャスト対象の式
}

// ---------------------------------------------------------------------
// Operators
// ---------------------------------------------------------------------

/// ## 二項演算子の種類
///
/// 二項演算子は、2つのオペランドを取る演算子です。
/// 例えば、`a + b` のような加算演算子 `+` は2つのオペランド `a` と `b` を取ります。
///
/// 二項演算子は、加算、減算、乗算、除算などがあります。
///
/// ```shot
/// 1 + 2
/// 3 - 4
/// 5 * 6
/// 7 / 8
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,      // 加算
    Subtract, // 減算
    Multiply, // 乗算
    Divide,   // 除算
}

// ---------------------------------------------------------------------
// Types and Literals
// ---------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Integer,  // 整数型
    Float,    // 浮動小数点型
    String,   // 文字列型
    Void,     // Void 型 (戻り値がない)
    Function, // 関数型
}

// リテラルの値の種類
#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Integer(i64),   // 整数リテラル
    Float(f64),     // 浮動小数点リテラル
    String(String), // 文字列リテラル
    None,           // Noneリテラル
}
