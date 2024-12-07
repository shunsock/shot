pub(crate) mod expression_evaluator;

use crate::virtual_machine::ast::{LiteralValue, Statement};
use crate::virtual_machine::evaluator::evaluation_error::EvaluationError;
use crate::virtual_machine::evaluator::statement_evaluator::expression_evaluator::evaluate_expression;
use crate::virtual_machine::evaluator::Evaluator;

pub(crate) fn evaluate_statement(
    evaluator: &mut Evaluator,
    statement: (usize, Statement),
) -> Result<(), EvaluationError> {
    // Statementを評価する
    match statement.1 {
        Statement::Expression(expr) => {
            // Expressionを評価する
            println!("{:?}", expr);
            Ok(())
        }
        Statement::DeclarationOfFunction(func) => {
            // 関数宣言を評価する
            Ok(())
        }
        _ => Ok(()),
    }
}
