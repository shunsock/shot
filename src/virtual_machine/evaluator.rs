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

    pub fn evaluate(&mut self) -> Result<LiteralValue, EvaluationError> {
        // ここで評価処理を行う
        for node in self.ast.statements.clone() {
            println!("{:?}", node);
            // Return文なら評価して OK(LiteralValue) を返す
            // それ以外の場合は次のステートメントを評価する (OKを返さずに次の評価を続ける)
        }
        // 最後までReturn文がなかった場合は None を返す
        // 自作関数の場合、Parserの時点でReturn文があることを保証しているので、ここでNoneを返すことはない
        // つまり、ここでNoneを返すのは、GlobalScopeの場合のみ
        Ok(LiteralValue::None)
    }
}
