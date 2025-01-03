use thiserror::Error;

#[derive(Debug, Clone, Error, PartialEq)]
pub enum EvaluationError {
    #[error("Function {name} not found at line {line}")]
    FunctionNotFound { name: String, line: usize },
    #[error("Variable {name} not found at line {line}")]
    VariableNotFound { name: String, line: usize },
    #[error("Variable {name} is already defined. You cannot reassign a variable at line {line}")]
    ReassignmentError { name: String, line: usize },
    #[error("Parameter {param_name} not found in function {function_name} at line {line}")]
    ParameterNotFound {
        function_name: String,
        param_name: String,
        line: usize,
    },
    #[error("Parameter type mismatch at line {line}. Function {function_name} expects {expected} but {actual} is given.")]
    ParameterTypeMismatch {
        function_name: String,
        param_name: String,
        line: usize,
        expected: String,
        actual: String,
    },
    #[error("Type Cast Error at line {line}. from_type: {from_type}, to_type: {to_type}")]
    InvalidTypeCast {
        line: usize,
        from_type: String,
        to_type: String,
    },
    #[error("Type Cast Error at line {line}. from_type: {from_type}, value: {value}")]
    FailedToTypeCast {
        line: usize,
        from_type: String,
        to_type: String,
        value: String,
    },
    #[error("Argument length error at line {line}. Function {function_name} expects {expected} arguments, but {actual} arguments are given.")]
    ArgumentLengthError {
        function_name: String,
        line: usize,
        expected: usize,
        actual: usize,
    },
    #[error("Unexpected error at line {line}. Please report this issue.")]
    UnexpectedError { line: usize },
}
