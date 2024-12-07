use crate::virtual_machine::ast::{ExpressionNode, LiteralValue};
use crate::virtual_machine::evaluator::evaluation_error::EvaluationError;
use crate::virtual_machine::evaluator::Evaluator;

pub fn evaluate_expression(
    evaluator: &mut Evaluator,
    expression: Box<ExpressionNode>,
) -> Result<LiteralValue, EvaluationError> {
    // Expressionを評価する
    Ok(LiteralValue::None)
}
