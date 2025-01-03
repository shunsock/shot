use crate::virtual_machine::ast::{LiteralValue, Type, TypeCastNode};
use crate::virtual_machine::evaluator::core::type_to_string;
use crate::virtual_machine::evaluator::evaluation_error::EvaluationError;
use crate::virtual_machine::evaluator::statement_evaluator::expression_evaluator::evaluate_expression;
use crate::virtual_machine::evaluator::Evaluator;

pub(crate) fn evaluate_type_cast(
    evaluator: &mut Evaluator,
    node: TypeCastNode,
) -> Result<LiteralValue, EvaluationError> {
    let value: LiteralValue = evaluate_expression(evaluator, *node.expression)?;
    match (node.from_type.clone(), node.to_type.clone()) {
        (Type::Integer, Type::Float) => {
            let float_value: f64 = value.to_string().parse().unwrap();
            Ok(LiteralValue::Float(float_value))
        }
        (Type::Integer, Type::String) => Ok(LiteralValue::String(value.to_string())),
        (Type::Float, Type::Integer) => {
            let int_value: i64 = value.to_string().parse().unwrap();
            Ok(LiteralValue::Integer(int_value))
        }
        (Type::Float, Type::String) => Ok(LiteralValue::String(value.to_string())),
        (Type::String, Type::Integer) => match value.to_string().parse() {
            Ok(int_value) => Ok(LiteralValue::Integer(int_value)),
            Err(_) => Err(EvaluationError::FailedToTypeCast {
                line: evaluator.line,
                from_type: type_to_string(node.from_type),
                to_type: type_to_string(node.to_type),
                value: value.to_string(),
            }),
        },
        (Type::String, Type::Float) => match value.to_string().parse() {
            Ok(float_value) => Ok(LiteralValue::Float(float_value)),
            Err(_) => Err(EvaluationError::FailedToTypeCast {
                line: evaluator.line,
                from_type: type_to_string(node.from_type),
                to_type: type_to_string(node.to_type),
                value: value.to_string(),
            }),
        },
        (_, _) => {
            let from_type_name: String = type_to_string(node.from_type);
            let to_type_name: String = type_to_string(node.to_type);
            Err(EvaluationError::InvalidTypeCast {
                line: evaluator.line,
                from_type: from_type_name,
                to_type: to_type_name,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::virtual_machine::ast::{ExpressionNode, LiteralNode, LiteralValue, Statement, Type, TypeCastNode};
    use crate::virtual_machine::evaluator::core::initialize_evaluator_with_custom_ast;
    use crate::virtual_machine::evaluator::statement_evaluator::expression_evaluator::type_cast_evaluator::evaluate_type_cast;
    use crate::virtual_machine::evaluator::Evaluator;

    /// 数値表現であるInt型をFloat型にキャストしようとした場合、正常にキャストされることを確認します。
    ///
    /// 1 as int -> float;  # 1.0
    #[test]
    fn test_type_cast_evaluator_can_cast_int_to_float() {
        let expected: LiteralValue = LiteralValue::Float(1.0);
        let literal_integer_expression: Box<ExpressionNode> =
            Box::new(ExpressionNode::Literal(Box::new(LiteralNode {
                value: LiteralValue::Integer(1),
            })));
        let type_cast_expression: TypeCastNode = TypeCastNode {
            from_type: Type::Integer,
            to_type: Type::Float,
            expression: literal_integer_expression.clone(),
        };
        let mut evaluator: Evaluator =
            initialize_evaluator_with_custom_ast(vec![Statement::Expression(
                *literal_integer_expression.clone(),
            )]);
        let actual: LiteralValue =
            evaluate_type_cast(&mut evaluator, type_cast_expression).unwrap();
        assert_eq!(actual, expected);
    }

    /// 数値表現であるInt型をString型にキャストしようとした場合、正常にキャストされることを確認します。
    ///
    /// 1 as int -> string;  # "1"
    #[test]
    fn test_type_cast_evaluator_can_cast_int_to_string() {
        let expected: LiteralValue = LiteralValue::String("1".to_string());
        let literal_integer_expression: Box<ExpressionNode> =
            Box::new(ExpressionNode::Literal(Box::new(LiteralNode {
                value: LiteralValue::Integer(1),
            })));
        let type_cast_expression: TypeCastNode = TypeCastNode {
            from_type: Type::Integer,
            to_type: Type::String,
            expression: literal_integer_expression.clone(),
        };
        let mut evaluator: Evaluator =
            initialize_evaluator_with_custom_ast(vec![Statement::Expression(
                *literal_integer_expression.clone(),
            )]);
        let actual: LiteralValue =
            evaluate_type_cast(&mut evaluator, type_cast_expression).unwrap();
        assert_eq!(actual, expected);
    }

    /// 数値表現であるFloat型をInt型にキャストしようとした場合、正常にキャストされることを確認します。
    ///
    /// 1.0 as float -> int;  # 1
    #[test]
    fn test_type_cast_evaluator_can_cast_float_to_int() {
        let expected: LiteralValue = LiteralValue::Integer(1);
        let literal_float_expression: Box<ExpressionNode> =
            Box::new(ExpressionNode::Literal(Box::new(LiteralNode {
                value: LiteralValue::Float(1.0),
            })));
        let type_cast_expression: TypeCastNode = TypeCastNode {
            from_type: Type::Float,
            to_type: Type::Integer,
            expression: literal_float_expression.clone(),
        };
        let mut evaluator: Evaluator =
            initialize_evaluator_with_custom_ast(vec![Statement::Expression(
                *literal_float_expression.clone(),
            )]);
        let actual: LiteralValue =
            evaluate_type_cast(&mut evaluator, type_cast_expression).unwrap();
        assert_eq!(actual, expected);
    }

    /// 数値表現であるFloat型をString型にキャストしようとした場合、正常にキャストされることを確認します。
    ///
    /// 1.0 as float -> string;  # "1"
    #[test]
    fn test_type_cast_evaluator_can_cast_float_to_string() {
        let expected: LiteralValue = LiteralValue::String("1".to_string());
        let literal_float_expression: Box<ExpressionNode> =
            Box::new(ExpressionNode::Literal(Box::new(LiteralNode {
                value: LiteralValue::Float(1.0),
            })));
        let type_cast_expression: Box<TypeCastNode> = Box::new(TypeCastNode {
            from_type: Type::Float,
            to_type: Type::String,
            expression: literal_float_expression.clone(),
        });
        let mut evaluator: Evaluator =
            initialize_evaluator_with_custom_ast(vec![Statement::Expression(
                *literal_float_expression.clone(),
            )]);
        let actual: LiteralValue =
            evaluate_type_cast(&mut evaluator, *type_cast_expression).unwrap();
        assert_eq!(actual, expected);
    }

    /// 数値表現であるString型をInt型にキャストしようとした場合、正常にキャストされることを確認します。
    ///
    /// "1" as string -> int;  # 1
    #[test]
    fn test_type_cast_evaluator_can_cast_numeric_string_to_int() {
        let expected: LiteralValue = LiteralValue::Integer(1);
        let literal_string_expression: Box<ExpressionNode> =
            Box::new(ExpressionNode::Literal(Box::new(LiteralNode {
                value: LiteralValue::String("1".to_string()),
            })));
        let type_cast_expression: TypeCastNode = TypeCastNode {
            from_type: Type::String,
            to_type: Type::Integer,
            expression: literal_string_expression.clone(),
        };
        let mut evaluator: Evaluator =
            initialize_evaluator_with_custom_ast(vec![Statement::Expression(
                *literal_string_expression.clone(),
            )]);
        let actual: LiteralValue =
            evaluate_type_cast(&mut evaluator, type_cast_expression).unwrap();
        assert_eq!(actual, expected);
    }

    /// 数値表現であるString型をFloat型にキャストしようとした場合、正常にキャストされることを確認します。
    ///
    /// "1" as string -> float;  # 1.0
    #[test]
    fn test_type_cast_evaluator_can_cast_numeric_string_to_float() {
        let expected: LiteralValue = LiteralValue::Float(1.0);
        let literal_string_expression: Box<ExpressionNode> =
            Box::new(ExpressionNode::Literal(Box::new(LiteralNode {
                value: LiteralValue::String("1".to_string()),
            })));
        let type_cast_expression: TypeCastNode = TypeCastNode {
            from_type: Type::String,
            to_type: Type::Float,
            expression: literal_string_expression.clone(),
        };
        let mut evaluator: Evaluator =
            initialize_evaluator_with_custom_ast(vec![Statement::Expression(
                *literal_string_expression.clone(),
            )]);
        let actual: LiteralValue =
            evaluate_type_cast(&mut evaluator, type_cast_expression).unwrap();
        assert_eq!(actual, expected);
    }

    /// 数値表現ではないString型をInt型にキャストしようとした場合、エラーが返されることを確認します。
    ///
    /// "a" as string -> int;  # Error
    #[test]
    fn test_type_cast_evaluator_fail_to_cast_non_numeric_string_to_int() {
        let literal_string_expression: Box<ExpressionNode> =
            Box::new(ExpressionNode::Literal(Box::new(LiteralNode {
                value: LiteralValue::String("a".to_string()),
            })));
        let type_cast_expression: TypeCastNode = TypeCastNode {
            from_type: Type::String,
            to_type: Type::Integer,
            expression: literal_string_expression.clone(),
        };
        let mut evaluator: Evaluator =
            initialize_evaluator_with_custom_ast(vec![Statement::Expression(
                *literal_string_expression.clone(),
            )]);
        let actual: Result<LiteralValue, _> =
            evaluate_type_cast(&mut evaluator, type_cast_expression);
        assert!(actual.is_err());
    }

    /// 数値表現ではないString型をFloat型にキャストしようとした場合、エラーが返されることを確認します。
    ///
    /// "a" as string -> float;  # Error
    #[test]
    fn test_type_cast_evaluator_fail_to_cast_non_numeric_string_to_float() {
        let literal_string_expression: Box<ExpressionNode> =
            Box::new(ExpressionNode::Literal(Box::new(LiteralNode {
                value: LiteralValue::String("a".to_string()),
            })));
        let type_cast_expression: TypeCastNode = TypeCastNode {
            from_type: Type::String,
            to_type: Type::Float,
            expression: literal_string_expression.clone(),
        };
        let mut evaluator: Evaluator =
            initialize_evaluator_with_custom_ast(vec![Statement::Expression(
                *literal_string_expression.clone(),
            )]);
        let actual: Result<LiteralValue, _> =
            evaluate_type_cast(&mut evaluator, type_cast_expression);
        assert!(actual.is_err());
    }

    /// 型キャストが定義されていない型の組み合わせでキャストしようとした場合、エラーが返されることを確認します。
    ///
    /// 1 as int -> function;  # Error
    #[test]
    fn test_type_cast_evaluator_fail_to_cast_invalid_type() {
        let literal_string_expression: Box<ExpressionNode> =
            Box::new(ExpressionNode::Literal(Box::new(LiteralNode {
                value: LiteralValue::String("a".to_string()),
            })));
        let type_cast_expression: TypeCastNode = TypeCastNode {
            from_type: Type::String,
            to_type: Type::Function,
            expression: literal_string_expression.clone(),
        };
        let mut evaluator: Evaluator =
            initialize_evaluator_with_custom_ast(vec![Statement::Expression(
                *literal_string_expression.clone(),
            )]);
        let actual: Result<LiteralValue, _> =
            evaluate_type_cast(&mut evaluator, type_cast_expression);
        assert!(actual.is_err());
    }
}
