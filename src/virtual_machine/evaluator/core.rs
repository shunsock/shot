use crate::virtual_machine::ast::AST;
use crate::virtual_machine::ast::{Statement, Type};
use crate::virtual_machine::evaluator::mapper::function_mapper::FunctionMapper;
use crate::virtual_machine::evaluator::mapper::variable_mapper::VariableMapper;
use crate::virtual_machine::evaluator::Evaluator;

#[cfg(test)]
pub(crate) fn initialize_evaluator_with_custom_ast(statements: Vec<Statement>) -> Evaluator {
    let mut ast: AST = AST::new();
    for stmt in statements {
        ast.push_statement(0, stmt);
    }
    Evaluator::new(ast, FunctionMapper::new(), VariableMapper::new())
}

pub(crate) fn type_to_string(t: Type) -> String {
    match t {
        Type::Float => "float".to_string(),
        Type::Function => "function".to_string(),
        Type::Integer => "int".to_string(),
        Type::String => "string".to_string(),
        Type::Void => "void".to_string(),
    }
}
