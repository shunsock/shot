use crate::virtual_machine::ast::{ExpressionNode, LiteralValue};
use crate::virtual_machine::evaluator::Evaluator;

pub fn evaluate_expression(
    evaluator: &mut Evaluator,
    expression: Box<ExpressionNode>,
) -> LiteralValue {
    match *expression {
        ExpressionNode::Literal(literal) => literal.value,
        // BinaryOperation
        // CallOfFunction
        // CallOfVariable
        // TypeCast
        _ => LiteralValue::None,
    }
}

#[cfg(test)]
mod tests {
    use crate::virtual_machine::ast::LiteralNode;
    use crate::virtual_machine::ast::{ExpressionNode, LiteralValue, Statement};
    use crate::virtual_machine::evaluator::core::initialize_evaluator_with_custom_ast;
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
        let actual: LiteralValue = evaluate_expression(&mut evaluator, expression);
        assert_eq!(actual, expected);
    }
}
