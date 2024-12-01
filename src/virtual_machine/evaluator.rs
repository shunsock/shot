mod function_mapper;
mod variable_mapper;

use function_mapper::FunctionMapper;
use variable_mapper::VariableMapper;

pub struct Evaluator {
    function_mapper: FunctionMapper,
    variable_mapper: VariableMapper,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            function_mapper: FunctionMapper::new(),
            variable_mapper: VariableMapper::new(),
        }
    }
}
