mod evaluation_error;
pub(crate) mod mapper;

use crate::virtual_machine::ast::{LiteralNode, LiteralValue, AST};
use crate::virtual_machine::evaluator::evaluation_error::EvaluationError;
use mapper::function_mapper::FunctionMapper;
use mapper::variable_mapper::VariableMapper;

pub struct Evaluator {
    ast: AST,
    function_mapper: FunctionMapper,
    variable_mapper: VariableMapper,
}

impl Evaluator {
    pub fn new(ast: AST, function_mapper: FunctionMapper, variable_mapper: VariableMapper) -> Self {
        Evaluator {
            ast,
            function_mapper,
            variable_mapper,
        }
    }

    pub fn evaluate(&mut self) -> Result<LiteralNode, EvaluationError> {
        // ここで評価処理を行う
        Ok(LiteralNode {
            value: LiteralValue::Integer(0),
        })
    }
}
