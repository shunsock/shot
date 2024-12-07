pub(crate) mod expression_evaluator;

use crate::virtual_machine::ast::LiteralValue;
use crate::virtual_machine::evaluator::evaluation_error::EvaluationError;
use crate::virtual_machine::evaluator::Evaluator;

pub(crate) fn evaluate_statement(
    evaluator: &mut Evaluator,
) -> Result<LiteralValue, EvaluationError> {
    // Statementを評価する
    Ok(LiteralValue::None)
}
