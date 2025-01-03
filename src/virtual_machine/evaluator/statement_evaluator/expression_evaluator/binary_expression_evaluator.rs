use crate::virtual_machine::ast::LiteralValue;
use crate::virtual_machine::ast::{BinaryOperationNode, BinaryOperator};
use crate::virtual_machine::evaluator::evaluation_error::EvaluationError;
use crate::virtual_machine::evaluator::statement_evaluator::expression_evaluator::evaluate_expression;
use crate::virtual_machine::evaluator::Evaluator;

pub(crate) fn evaluate_binary_expression(
    evaluator: &mut Evaluator,
    binary_expression: BinaryOperationNode,
) -> Result<LiteralValue, EvaluationError> {
    let left: LiteralValue = evaluate_expression(evaluator, *binary_expression.left)?;
    let right: LiteralValue = evaluate_expression(evaluator, *binary_expression.right)?;
    match binary_expression.operator {
        BinaryOperator::Add => add(left, right),
        BinaryOperator::Subtract => subtract(left, right),
        BinaryOperator::Multiply => multiply(left, right),
        BinaryOperator::Divide => divide(left, right),
    }
}

/// 足し算を行う関数
///
/// # Arguments
///
/// * `left` - 左辺の値
/// * `right` - 右辺の値
///
/// # Returns
///
/// * `Result<LiteralValue, EvaluationError>` - 足し算の結果
fn add(left: LiteralValue, right: LiteralValue) -> Result<LiteralValue, EvaluationError> {
    match (left.clone(), right.clone()) {
        (LiteralValue::Integer(left_value), LiteralValue::Integer(right_value)) => {
            Ok(LiteralValue::Integer(left_value + right_value))
        }
        (LiteralValue::Integer(left_value), LiteralValue::Float(right_value)) => {
            Ok(LiteralValue::Float(left_value as f64 + right_value))
        }
        (LiteralValue::Float(left_value), LiteralValue::Integer(right_value)) => {
            Ok(LiteralValue::Float(left_value + right_value as f64))
        }
        (LiteralValue::Float(left_value), LiteralValue::Float(right_value)) => {
            Ok(LiteralValue::Float(left_value + right_value))
        }
        (LiteralValue::String(left_value), LiteralValue::String(right_value)) => Ok(
            LiteralValue::String(format!("{}{}", left_value, right_value)),
        ),
        _ => Err(EvaluationError::InvalidBinaryOperation {
            line: 0,
            operator: "+".to_string(),
            left: left.to_string(),
            right: right.to_string(),
        }),
    }
}

/// 引き算を行う関数
///
/// # Arguments
///
/// * `left` - 左辺の値
/// * `right` - 右辺の値
///
/// # Returns
///
/// * `Result<LiteralValue, EvaluationError>` - 引き算の結果
fn subtract(left: LiteralValue, right: LiteralValue) -> Result<LiteralValue, EvaluationError> {
    match (left.clone(), right.clone()) {
        (LiteralValue::Integer(left_value), LiteralValue::Integer(right_value)) => {
            Ok(LiteralValue::Integer(left_value - right_value))
        }
        (LiteralValue::Integer(left_value), LiteralValue::Float(right_value)) => {
            Ok(LiteralValue::Float(left_value as f64 - right_value))
        }
        (LiteralValue::Float(left_value), LiteralValue::Integer(right_value)) => {
            Ok(LiteralValue::Float(left_value - right_value as f64))
        }
        (LiteralValue::Float(left_value), LiteralValue::Float(right_value)) => {
            Ok(LiteralValue::Float(left_value - right_value))
        }
        _ => Err(EvaluationError::InvalidBinaryOperation {
            line: 0,
            operator: "-".to_string(),
            left: left.to_string(),
            right: right.to_string(),
        }),
    }
}

/// 掛け算を行う関数
///
/// # Arguments
///
/// * `left` - 左辺の値
/// * `right` - 右辺の値
///
/// # Returns
///
/// * `Result<LiteralValue, EvaluationError>` - 掛け算の結果
fn multiply(left: LiteralValue, right: LiteralValue) -> Result<LiteralValue, EvaluationError> {
    match (left.clone(), right.clone()) {
        (LiteralValue::Integer(left_value), LiteralValue::Integer(right_value)) => {
            Ok(LiteralValue::Integer(left_value * right_value))
        }
        (LiteralValue::Integer(left_value), LiteralValue::Float(right_value)) => {
            Ok(LiteralValue::Float(left_value as f64 * right_value))
        }
        (LiteralValue::Float(left_value), LiteralValue::Integer(right_value)) => {
            Ok(LiteralValue::Float(left_value * right_value as f64))
        }
        (LiteralValue::Float(left_value), LiteralValue::Float(right_value)) => {
            Ok(LiteralValue::Float(left_value * right_value))
        }
        _ => Err(EvaluationError::InvalidBinaryOperation {
            line: 0,
            operator: "*".to_string(),
            left: left.to_string(),
            right: right.to_string(),
        }),
    }
}

/// 割り算を行う関数
///
/// # Arguments
///
/// * `left` - 左辺の値
/// * `right` - 右辺の値
///
/// # Returns
///
/// * `Result<LiteralValue, EvaluationError>` - 割り算の結果
fn divide(left: LiteralValue, right: LiteralValue) -> Result<LiteralValue, EvaluationError> {
    match (left.clone(), right.clone()) {
        (_, LiteralValue::Integer(0)) => Err(EvaluationError::UnexpectedError { line: 0 }),
        (_, LiteralValue::Float(0.0)) => Err(EvaluationError::UnexpectedError { line: 0 }),
        (LiteralValue::Integer(left_value), LiteralValue::Integer(right_value)) => {
            Ok(LiteralValue::Integer(left_value / right_value))
        }
        (LiteralValue::Integer(left_value), LiteralValue::Float(right_value)) => {
            Ok(LiteralValue::Float(left_value as f64 / right_value))
        }
        (LiteralValue::Float(left_value), LiteralValue::Integer(right_value)) => {
            Ok(LiteralValue::Float(left_value / right_value as f64))
        }
        _ => Err(EvaluationError::InvalidBinaryOperation {
            line: 0,
            operator: "/".to_string(),
            left: left.to_string(),
            right: right.to_string(),
        }),
    }
}
