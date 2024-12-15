use crate::virtual_machine::ast::Statement;
use crate::virtual_machine::ast::AST;
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
