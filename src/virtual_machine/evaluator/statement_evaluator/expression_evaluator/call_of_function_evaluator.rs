use crate::virtual_machine::ast::AST;
use crate::virtual_machine::ast::{
    ExpressionNode, FunctionDeclarationNode, LiteralNode, VariableDeclarationNode,
};
use crate::virtual_machine::ast::{FunctionCallNode, LiteralValue, Type};
use crate::virtual_machine::evaluator::evaluation_error::EvaluationError;
use crate::virtual_machine::evaluator::mapper::function_mapper::FunctionMapper;
use crate::virtual_machine::evaluator::mapper::variable_mapper::VariableMapper;
use crate::virtual_machine::evaluator::statement_evaluator::expression_evaluator::evaluate_expression;
use crate::virtual_machine::evaluator::Evaluator;

pub(crate) fn call_of_function_evaluator(
    evaluator: &mut Evaluator,
    node: FunctionCallNode,
) -> Result<LiteralValue, EvaluationError> {
    // 関数呼び出しNodeから呼び出した関数名と引数を取得
    let calling_function_name: String = node.name.clone();
    let calling_function_arguments: Vec<(String, ExpressionNode)> = node.arguments.clone();

    // 関数宣言Nodeから呼び出された関数の情報を取得
    let called_function: FunctionDeclarationNode = evaluator
        .function_mapper
        .get(&calling_function_name, evaluator.line)?;
    let called_function_arguments: Vec<(String, Type)> = called_function.params.clone();

    // Validate
    let params: Vec<(String, Type, LiteralValue)> = validate_params(
        evaluator,
        calling_function_name.clone(),
        calling_function_arguments.clone(),
        called_function_arguments.clone(),
    )?;

    // 関数呼び出しのためのスコープを設定
    let mut ast: AST = AST::new();
    for stmt in called_function.body.clone() {
        ast.push_statement(evaluator.line, stmt);
    }
    let function_scope_evaluator: &mut Evaluator =
        &mut Evaluator::new(ast, FunctionMapper::new(), VariableMapper::new());
    setup_scope(function_scope_evaluator, params)?;
    let function_return_value: LiteralValue = function_scope_evaluator.evaluate()?;

    Ok(function_return_value)
}

/// 関数呼び出しの引数の情報を元に、呼び出した関数のbodyで用いるEvaluatorとVariableMapperを作成
///
/// ## Arguments
///
/// * `evaluator` - Evaluator
/// * `function_params` - 関数呼び出しの引数の情報
///
/// ## Returns
///
/// * `Result<(), EvaluationError>` - Ok(())
///
/// ## Raises
///
/// * `EvaluationError` - 変数のセットに失敗（同じ変数名が存在する場合など）
fn setup_scope(
    evaluator: &mut Evaluator,
    function_params: Vec<(String, Type, LiteralValue)>,
) -> Result<(), EvaluationError> {
    for param in function_params {
        let node: VariableDeclarationNode = VariableDeclarationNode {
            name: param.0.clone(),
            var_type: param.1.clone(),
            value: Box::new(generate_literal_node(param.2)),
        };
        evaluator.variable_mapper.set(evaluator.line, node)?;
    }
    Ok(())
}

fn generate_literal_node(value: LiteralValue) -> ExpressionNode {
    ExpressionNode::Literal(Box::new(LiteralNode { value }))
}

/// 関数呼び出しの引数の情報を元に、呼び出された関数の引数の情報を検証
///
/// - 引数の数が一致しているか
/// - 引数の名前が一致しているか
/// - 型が一致しているかを検証
///
/// ## Arguments
///
/// * `evaluator` - Evaluator
/// * `calling_function_name` - 関数呼び出しの関数名
/// * `calling_function_params` - 関数呼び出しの引数の情報
/// * `called_function_params` - 呼び出された関数の引数の情報
///
/// ## Returns
///
/// * `Result<Vec<(String, Type, LiteralValue)>, EvaluationError>` - Ok(Vec<(String, Type, LiteralValue)>)
///
/// ## Raises
///
/// * `EvaluationError::ArgumentLengthError` - 引数の数が一致しない場合
/// * `EvaluationError::ParameterTypeMismatch` - 引数の型が一致しない場合
/// * `EvaluationError::ParameterNotFound` - 引数の名前が一致しない場合
fn validate_params(
    evaluator: &mut Evaluator,
    calling_function_name: String,
    calling_function_params: Vec<(String, ExpressionNode)>,
    called_function_params: Vec<(String, Type)>,
) -> Result<Vec<(String, Type, LiteralValue)>, EvaluationError> {
    // 引数の数が一致しているかチェック
    if calling_function_params.len() != called_function_params.len() {
        return Err(EvaluationError::ArgumentLengthError {
            function_name: calling_function_name.clone(),
            expected: called_function_params.len(),
            actual: calling_function_params.len(),
            line: evaluator.line,
        });
    }

    // 戻り値の定義
    let mut result: Vec<(String, Type, LiteralValue)> = vec![];

    // 引数の型が一致しているかチェック
    let mut i: usize = 0;
    while i < calling_function_params.len() {
        let calling_function_param_name: String = calling_function_params[i].0.clone();
        let calling_function_param_value: LiteralValue =
            evaluate_expression(evaluator, calling_function_params[i].1.clone())?;
        let calling_function_param_type: Type =
            literal_to_type(calling_function_param_value.clone());

        // indexを使った処理はここで終わるので、インクリメント
        i += 1;

        match search_argument_name_in_called_function_params(
            calling_function_param_name.clone(),
            called_function_params.clone(),
        ) {
            Some(called_param_type) => {
                // パラメータの型と一致していない
                // let f: fn = (x: int): Void { return none; };
                // f(x: 1.0); // ParameterTypeMismatch
                if calling_function_param_type.clone() != called_param_type.clone() {
                    return Err(EvaluationError::ParameterTypeMismatch {
                        function_name: calling_function_name.clone(),
                        param_name: calling_function_param_name.clone(),
                        line: evaluator.line,
                        expected: called_param_type.to_string(),
                        actual: calling_function_param_type.to_string(),
                    });
                }
                result.push((
                    calling_function_param_name,
                    called_param_type,
                    calling_function_param_value,
                ));
            }
            // パラメータの名前付き引数の名前解決に失敗
            // let f: fn = (x: int): Void { return none; };
            // f(z: 1); // ParameterNotFound
            None => {
                return Err(EvaluationError::ParameterNotFound {
                    function_name: calling_function_name.clone(),
                    param_name: calling_function_param_name.clone(),
                    line: evaluator.line,
                });
            }
        }
    }
    Ok(result)
}

/// 呼び出しに用いられた引数の名前が、呼び出された関数の引数の中に存在するかを検証
///
/// ## Arguments
///
/// * `name` - 引数の名前
/// * `params` - 呼び出された関数の引数の情報
///
/// ## Returns
///
/// * `Option<Type>` - 呼び出された関数の引数の型
fn search_argument_name_in_called_function_params(
    name: String,
    params: Vec<(String, Type)>,
) -> Option<Type> {
    for param in params {
        if param.0 == name {
            return Some(param.1);
        }
    }
    None
}

fn literal_to_type(value: LiteralValue) -> Type {
    match value {
        LiteralValue::Integer(_) => Type::Integer,
        LiteralValue::Float(_) => Type::Float,
        LiteralValue::String(_) => Type::String,
        LiteralValue::None => Type::Void,
    }
}

#[cfg(test)]
mod tests {
    use crate::virtual_machine::ast::{Statement, VariableCallNode};
    use crate::virtual_machine::ast::{FunctionCallNode, FunctionDeclarationNode, LiteralNode};
    use crate::virtual_machine::ast::{ExpressionNode, LiteralValue, Type, VariableDeclarationNode, AST};
    use crate::virtual_machine::evaluator::core::initialize_evaluator_with_custom_ast;
    use crate::virtual_machine::evaluator::evaluation_error::EvaluationError;
    use crate::virtual_machine::evaluator::Evaluator;
    use crate::virtual_machine::evaluator::mapper::function_mapper::FunctionMapper;
    use crate::virtual_machine::evaluator::mapper::variable_mapper::VariableMapper;
    use crate::virtual_machine::evaluator::statement_evaluator::expression_evaluator::call_of_function_evaluator::{call_of_function_evaluator, setup_scope, validate_params};

    /// setup_scope 関数は引数の情報を元にevaluatorを正しく初期化する
    ///
    /// let f: fn = (x: int): int { return x; };
    /// f(x: 1); -- VariableMapperにxがセットされる
    #[test]
    fn test_setup_scope() {
        // 期待される値
        let expected: VariableDeclarationNode = VariableDeclarationNode {
            name: "x".to_string(),
            var_type: Type::Integer,
            value: Box::new(ExpressionNode::Literal(Box::new(LiteralNode {
                value: LiteralValue::Integer(1),
            }))),
        };

        // テスト対象のセットアップ
        let ast: AST = AST::new();
        let function_scope_evaluator: &mut Evaluator =
            &mut Evaluator::new(ast, FunctionMapper::new(), VariableMapper::new());
        let params: Vec<(String, Type, LiteralValue)> =
            vec![("x".to_string(), Type::Integer, LiteralValue::Integer(1))];

        // テスト対象の実行
        let r: Result<(), EvaluationError> = setup_scope(function_scope_evaluator, params);

        // 結果の検証
        assert_eq!(r, Ok(()));
        let variable_data: VariableDeclarationNode =
            match function_scope_evaluator.variable_mapper.get("x", 0) {
                Ok(v) => v,
                Err(_) => panic!("test_setup_scope failed: 作成した変数が見つかりません"),
            };
        assert_eq!(variable_data, expected);
    }

    /// validate_params 関数は正しい関数への入力を正常に検証する
    ///
    /// let f: fn = (x: int, y: float): int { return x; };
    /// f(x: 1, y: 1.0); -- 正常な関数呼び出し
    #[test]
    fn test_validate_params() {
        // 期待される値
        let expected: Vec<(String, Type, LiteralValue)> = vec![
            ("x".to_string(), Type::Integer, LiteralValue::Integer(1)),
            ("y".to_string(), Type::Float, LiteralValue::Float(1.0)),
        ];

        // テスト対象のセットアップ
        let mut evaluator: Evaluator = initialize_evaluator_with_custom_ast(vec![]);
        let calling_function_name: String = "f".to_string();
        let calling_function_params: Vec<(String, ExpressionNode)> = vec![
            (
                "x".to_string(),
                ExpressionNode::Literal(Box::new(LiteralNode {
                    value: LiteralValue::Integer(1),
                })),
            ),
            (
                "y".to_string(),
                ExpressionNode::Literal(Box::new(LiteralNode {
                    value: LiteralValue::Float(1.0),
                })),
            ),
        ];
        let called_function_params: Vec<(String, Type)> = vec![
            ("x".to_string(), Type::Integer),
            ("y".to_string(), Type::Float),
        ];

        // テスト対象の実行
        let r: Result<Vec<(String, Type, LiteralValue)>, EvaluationError> = validate_params(
            &mut evaluator,
            calling_function_name.clone(),
            calling_function_params.clone(),
            called_function_params.clone(),
        );

        // 結果の検証
        assert_eq!(r, Ok(expected));
    }

    /// validate_params 関数は引数の数が一致しない場合、エラーを返す
    ///
    /// let f: fn = (x: int, y: float): int { return x; };
    /// f(x: 1); -- 引数の数が一致しない
    #[test]
    fn test_validate_params_argument_length_error() {
        // テスト対象のセットアップ
        let mut evaluator: Evaluator = initialize_evaluator_with_custom_ast(vec![]);
        let calling_function_name: String = "f".to_string();
        // -- let f: fn = (x: int, y: float): int { return x; }; の引数
        let called_function_params: Vec<(String, Type)> = vec![
            ("x".to_string(), Type::Integer),
            ("y".to_string(), Type::Float),
        ];
        // -- let f: fn = (x: int, y: float): int { return x; }; の引数のうち、yがない
        let calling_function_params: Vec<(String, ExpressionNode)> = vec![(
            "x".to_string(),
            ExpressionNode::Literal(Box::new(LiteralNode {
                value: LiteralValue::Integer(1),
            })),
        )];

        // テスト対象の実行
        let r: Result<Vec<(String, Type, LiteralValue)>, EvaluationError> = validate_params(
            &mut evaluator,
            calling_function_name.clone(),
            calling_function_params.clone(),
            called_function_params.clone(),
        );

        // 結果の検証
        assert_eq!(
            r,
            Err(EvaluationError::ArgumentLengthError {
                function_name: calling_function_name.clone(),
                expected: 2,
                actual: 1,
                line: 0
            })
        );
    }

    /// validate_params 関数は引数の名前が一致しない場合、エラーを返す
    ///
    /// let f: fn = (x: int, y: float): int { return x; };
    /// f(z: 1, y: 1.0); -- 引数の名前が一致しない
    #[test]
    fn test_validate_params_parameter_not_found() {
        // テスト対象のセットアップ
        let mut evaluator: Evaluator = initialize_evaluator_with_custom_ast(vec![]);
        let calling_function_name: String = "f".to_string();
        // -- let f: fn = (x: int, y: float): int { return x; }; の引数
        let called_function_params: Vec<(String, Type)> = vec![
            ("x".to_string(), Type::Integer),
            ("y".to_string(), Type::Float),
        ];
        // -- zは存在しない引数
        let calling_function_params: Vec<(String, ExpressionNode)> = vec![
            (
                "z".to_string(),
                ExpressionNode::Literal(Box::new(LiteralNode {
                    value: LiteralValue::Integer(1),
                })),
            ),
            (
                "y".to_string(),
                ExpressionNode::Literal(Box::new(LiteralNode {
                    value: LiteralValue::Float(1.0),
                })),
            ),
        ];

        // テスト対象の実行
        let r: Result<Vec<(String, Type, LiteralValue)>, EvaluationError> = validate_params(
            &mut evaluator,
            calling_function_name.clone(),
            calling_function_params.clone(),
            called_function_params.clone(),
        );

        // 結果の検証
        assert_eq!(
            r,
            Err(EvaluationError::ParameterNotFound {
                function_name: calling_function_name.clone(),
                param_name: "z".to_string(),
                line: 0
            })
        );
    }

    /// validate_params 関数は引数の型が一致しない場合、エラーを返す
    ///
    /// let f: fn = (x: int, y: float): int { return x; };
    /// f(x: 1, y: 1); -- 引数yの型が一致しない
    #[test]
    fn test_validate_params_parameter_type_mismatch() {
        // テスト対象のセットアップ
        let mut evaluator: Evaluator = initialize_evaluator_with_custom_ast(vec![]);
        let calling_function_name: String = "f".to_string();
        // -- let f: fn = (x: int, y: float): int { return x; }; の引数
        let called_function_params: Vec<(String, Type)> = vec![
            ("x".to_string(), Type::Integer),
            ("y".to_string(), Type::Float),
        ];
        // -- yの型が一致しない: Expected: float, Actual: int
        let calling_function_params: Vec<(String, ExpressionNode)> = vec![
            (
                "x".to_string(),
                ExpressionNode::Literal(Box::new(LiteralNode {
                    value: LiteralValue::Integer(1),
                })),
            ),
            (
                "y".to_string(),
                ExpressionNode::Literal(Box::new(LiteralNode {
                    value: LiteralValue::Integer(1),
                })),
            ),
        ];

        // テスト対象の実行
        let r: Result<Vec<(String, Type, LiteralValue)>, EvaluationError> = validate_params(
            &mut evaluator,
            calling_function_name.clone(),
            calling_function_params.clone(),
            called_function_params.clone(),
        );

        // 結果の検証
        assert_eq!(
            r,
            Err(EvaluationError::ParameterTypeMismatch {
                function_name: calling_function_name.clone(),
                param_name: "y".to_string(),
                line: 0,
                expected: Type::Float.to_string(),
                actual: Type::Integer.to_string()
            })
        );
    }

    /// evaluate_call_of_function 関数は関数呼び出しを正常に評価する
    ///
    /// let f: fn = (): int { return 0; };
    /// f(); -- 関数呼び出し
    #[test]
    fn test_evaluate_call_of_function() {
        // 期待される値
        let expected: LiteralValue = LiteralValue::Integer(0);

        // テスト対象のセットアップ
        let mut function_mapper: FunctionMapper = FunctionMapper::new();
        match function_mapper.set(
            0,
            // -- let f: fn = (): int { return 0; };
            FunctionDeclarationNode {
                name: "f".to_string(),
                params: vec![],
                return_type: Type::Integer,
                body: vec![Statement::Return(Box::new(ExpressionNode::Literal(
                    Box::new(LiteralNode {
                        value: LiteralValue::Integer(0),
                    }),
                )))],
            },
        ) {
            Ok(v) => v,
            Err(_) => panic!("test_evaluate_call_of_function failed: 関数の登録に失敗しました"),
        };
        let mut evaluator: Evaluator =
            Evaluator::new(AST::new(), function_mapper, VariableMapper::new());

        // -- f(); の関数呼び出し
        let node: FunctionCallNode = FunctionCallNode {
            name: "f".to_string(),
            arguments: vec![],
        };

        // テスト対象の実行
        let r: Result<LiteralValue, EvaluationError> =
            call_of_function_evaluator(&mut evaluator, node);
        let returned_value: LiteralValue = match r {
            Ok(v) => v,
            Err(_) => panic!("test_evaluate_call_of_function failed: 関数呼び出しに失敗しました"),
        };

        // 結果の検証
        assert_eq!(returned_value, expected);
    }

    /// evaluate_call_of_function 関数は引数を持つ関数呼び出しを正常に評価する
    ///
    /// let f: fn = (x: int, y: float): int { return x; };
    /// f(x: 1, y: 2.0); -- 引数を持つ関数呼び出し
    #[test]
    fn test_evaluate_call_of_function_with_arguments() {
        // 期待される値
        let expected: LiteralValue = LiteralValue::Integer(1);

        // テスト対象のセットアップ
        let mut function_mapper: FunctionMapper = FunctionMapper::new();
        match function_mapper.set(
            0,
            // -- let f: fn = (x: int, y: float): int { return x; };
            FunctionDeclarationNode {
                name: "f".to_string(),
                params: vec![
                    ("x".to_string(), Type::Integer),
                    ("y".to_string(), Type::Float),
                ],
                return_type: Type::Integer,
                body: vec![Statement::Return(Box::new(ExpressionNode::CallOfVariable(
                    Box::new(VariableCallNode {
                        name: "x".to_string(),
                    }),
                )))],
            },
        ) {
            Ok(v) => v,
            Err(_) => panic!(
                "test_evaluate_call_of_function_with_arguments failed: 関数の登録に失敗しました"
            ),
        };
        let mut evaluator: Evaluator =
            Evaluator::new(AST::new(), function_mapper, VariableMapper::new());

        // -- f(x: 1, y: 2.0); の関数呼び出し
        let node: FunctionCallNode = FunctionCallNode {
            name: "f".to_string(),
            arguments: vec![
                (
                    "x".to_string(),
                    ExpressionNode::Literal(Box::new(LiteralNode {
                        value: LiteralValue::Integer(1),
                    })),
                ),
                (
                    "y".to_string(),
                    ExpressionNode::Literal(Box::new(LiteralNode {
                        value: LiteralValue::Float(2.0),
                    })),
                ),
            ],
        };

        // テスト対象の実行
        let r: Result<LiteralValue, EvaluationError> =
            call_of_function_evaluator(&mut evaluator, node);
        let returned_value: LiteralValue = match r {
            Ok(v) => v,
            Err(_) => panic!(
                "test_evaluate_call_of_function_with_arguments failed: 関数呼び出しに失敗しました"
            ),
        };

        // 結果の検証
        assert_eq!(returned_value, expected);
    }
}
