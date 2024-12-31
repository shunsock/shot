mod call_of_variable_evaluator;
mod type_cast_evaluator;

use crate::virtual_machine::ast::{ExpressionNode, LiteralValue};
use crate::virtual_machine::evaluator::evaluation_error::EvaluationError;
use crate::virtual_machine::evaluator::Evaluator;
use crate::virtual_machine::evaluator::statement_evaluator::expression_evaluator::type_cast_evaluator::evaluate_type_cast;

pub fn evaluate_expression(
    evaluator: &mut Evaluator,
    expression: Box<ExpressionNode>,
) -> Result<LiteralValue, EvaluationError> {
    match *expression {
        ExpressionNode::Literal(literal) => Ok(literal.value),
        // BinaryOperation
        // CallOfFunction
        // CallOfVariable
        ExpressionNode::CallOfVariable(node) => {
            call_of_variable_evaluator::call_of_variable(evaluator, *node)
        }
        // TypeCast
        ExpressionNode::TypeCast(node) => evaluate_type_cast(evaluator, node),
        _ => Ok(LiteralValue::None),
    }
}

#[cfg(test)]
mod tests {
    use crate::virtual_machine::ast::LiteralNode;
    use crate::virtual_machine::ast::{ExpressionNode, LiteralValue, Statement};
    use crate::virtual_machine::evaluator::core::initialize_evaluator_with_custom_ast;
    use crate::virtual_machine::evaluator::evaluation_error::EvaluationError;
    use crate::virtual_machine::evaluator::statement_evaluator::expression_evaluator::evaluate_expression;
    use crate::virtual_machine::evaluator::Evaluator;

    #[test]
    fn test_evaluate_expression() {
        let expected = LiteralValue::Integer(1);
        let expression: Box<ExpressionNode> =
            Box::new(ExpressionNode::Literal(Box::new(LiteralNode {
                value: LiteralValue::Integer(1),
            })));
        let mut evaluator: Evaluator =
            initialize_evaluator_with_custom_ast(vec![Statement::Expression(*expression.clone())]);
        let actual: Result<LiteralValue, EvaluationError> =
            evaluate_expression(&mut evaluator, expression);
        assert_eq!(actual, Ok(expected.clone()));
        assert_eq!(actual.unwrap(), expected);
    }
}
