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
        let calling_function_param_name: String = called_function_params[i].0.clone();
        let calling_function_param_value: LiteralValue =
            evaluate_expression(evaluator, calling_function_params[i].1.clone())?;
        let calling_function_param_type: Type =
            literal_to_type(calling_function_param_value.clone());
        let called_function_param_type: Type = called_function_params[i].1.clone();

        // indexを使った処理はここで終わるので、インクリメント
        i += 1;

        match search_argument_name_in_called_function_params(
            calling_function_param_name.clone(),
            called_function_params.clone(),
        ) {
            Some(param_type) => {
                // パラメータの型と一致していない
                // let f: fn = (x: int): Void { return none; };
                // f(x: 1.0); // ParameterTypeMismatch
                if called_function_param_type.clone() != param_type.clone() {
                    return Err(EvaluationError::ParameterTypeMismatch {
                        function_name: calling_function_name.clone(),
                        param_name: calling_function_param_name.clone(),
                        line: evaluator.line,
                        expected: calling_function_param_type.to_string(),
                        actual: param_type.to_string(),
                    });
                }
                result.push((
                    calling_function_param_name,
                    param_type,
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
