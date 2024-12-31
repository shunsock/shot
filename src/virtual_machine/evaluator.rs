mod core;
mod evaluation_error;
pub(crate) mod mapper;
mod statement_evaluator;

use crate::virtual_machine::ast::{LiteralValue, Statement, AST};
use crate::virtual_machine::evaluator::evaluation_error::EvaluationError;
use mapper::function_mapper::FunctionMapper;
use mapper::variable_mapper::VariableMapper;
use statement_evaluator::evaluate_statement;
use statement_evaluator::expression_evaluator::evaluate_expression;

pub struct Evaluator {
    ast: AST,
    line: usize,
    function_mapper: FunctionMapper,
    variable_mapper: VariableMapper,
}

impl Evaluator {
    pub fn new(ast: AST, function_mapper: FunctionMapper, variable_mapper: VariableMapper) -> Self {
        Evaluator {
            ast,
            line: 0,
            function_mapper,
            variable_mapper,
        }
    }

    pub fn evaluate(&mut self) -> Result<LiteralValue, EvaluationError> {
        // ここで評価処理を行う
        for stmt in self.ast.statements.clone() {
            self.line = stmt.0;

            // Return文なら評価して OK(LiteralValue) を返す
            // それ以外の場合は次のステートメントを評価する (OKを返さずに次の評価を続ける)
            if let Statement::Return(expr) = stmt.1.clone() {
                return evaluate_expression(self, *expr);
            }
            evaluate_statement(self, stmt.1)?;
        }
        // 最後までReturn文がなかった場合は None を返す
        // 自作関数の場合、Parserの時点でReturn文があることを保証しているので、ここでNoneを返すことはない
        // つまり、ここでNoneを返すのは、GlobalScopeの場合のみ
        Ok(LiteralValue::None)
    }
}
