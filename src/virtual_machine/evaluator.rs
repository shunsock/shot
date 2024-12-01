mod mapper;

use mapper::function_mapper::FunctionMapper;
use mapper::variable_mapper::VariableMapper;

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
