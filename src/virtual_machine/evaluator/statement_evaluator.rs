pub(crate) mod expression_evaluator;

use crate::virtual_machine::ast::Statement;
use crate::virtual_machine::evaluator::evaluation_error::EvaluationError;
use crate::virtual_machine::evaluator::evaluation_error::EvaluationError::UnexpectedError;
use crate::virtual_machine::evaluator::statement_evaluator::expression_evaluator::evaluate_expression;
use crate::virtual_machine::evaluator::Evaluator;

pub(crate) fn evaluate_statement(
    evaluator: &mut Evaluator,
    statement: Statement,
) -> Result<(), EvaluationError> {
    // Statementを評価する
    match statement.clone() {
        Statement::Expression(expr) => {
            // Expressionを評価する
            evaluate_expression(evaluator, Box::new(expr))?;
            Ok(())
        }
        Statement::DeclarationOfFunction(func) => {
            evaluator
                .function_mapper
                .set(evaluator.line, *func.clone())?;
            Ok(())
        }
        Statement::DeclarationOfVariable(var) => {
            evaluator
                .variable_mapper
                .set(evaluator.line, *var.clone())?;
            Ok(())
        }
        _ => Err(UnexpectedError {
            line: evaluator.line,
        }),
    }
}
