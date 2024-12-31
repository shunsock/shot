use crate::virtual_machine::ast::VariableDeclarationNode;
use crate::virtual_machine::ast::{LiteralValue, VariableCallNode};
use crate::virtual_machine::evaluator::evaluation_error::EvaluationError;
use crate::virtual_machine::evaluator::statement_evaluator::expression_evaluator::evaluate_expression;
use crate::virtual_machine::evaluator::Evaluator;

/// 変数呼び出しを評価する
///
/// 変数呼び出しは、変数の値を取得する処理です。
///
/// ## Note
/// この関数はvariable_mapperとevaluate_expressionに依存しています
/// 両方の関数はテストされているため、この関数ではテストを行いません
pub(crate) fn call_of_variable(
    evaluator: &mut Evaluator,
    node: VariableCallNode,
) -> Result<LiteralValue, EvaluationError> {
    let var: VariableDeclarationNode = evaluator.variable_mapper.get(&node.name, evaluator.line)?;
    let literal_value: LiteralValue = evaluate_expression(evaluator, *var.value)?;
    Ok(literal_value)
}
