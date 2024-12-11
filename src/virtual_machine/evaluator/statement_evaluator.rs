pub(crate) mod expression_evaluator;

use crate::virtual_machine::ast::LiteralValue;
use crate::virtual_machine::ast::Statement;
use crate::virtual_machine::evaluator::evaluation_error::EvaluationError;
use crate::virtual_machine::evaluator::evaluation_error::EvaluationError::UnexpectedError;
use crate::virtual_machine::evaluator::Evaluator;

pub(crate) fn evaluate_statement(
    evaluator: &mut Evaluator,
    statement: Statement,
) -> Result<LiteralValue, EvaluationError> {
    // Statementを評価する
    match statement.clone() {
        Statement::Expression(expr) => {
            // Expressionを評価する
            let _ = expression_evaluator::evaluate_expression(evaluator, Box::new(expr))?;
            Ok(LiteralValue::None)
        }
        Statement::DeclarationOfFunction(func) => Ok(LiteralValue::None),
        Statement::DeclarationOfVariable(var) => Ok(LiteralValue::None),
        _ => Err(UnexpectedError {
            line: evaluator.line,
        }),
    }
}
