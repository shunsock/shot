use crate::virtual_machine::ast::VariableDeclarationNode;
use crate::virtual_machine::evaluator::evaluation_error::EvaluationError;
use std::collections::HashMap;

pub(crate) struct VariableMapper {
    map: HashMap<String, VariableDeclarationNode>,
}

impl VariableMapper {
    pub fn new() -> Self {
        VariableMapper {
            map: Default::default(),
        }
    }

    /// 変数を取得する
    ///
    /// # Arguments
    /// - `name` - 変数名
    /// - `line` - 行番号
    ///
    /// # Returns
    /// - `Result<VariableDeclarationNode, EvaluationError>` - Map結果
    ///   - `VariableDeclarationNode` - 変数の定義
    ///   - `EvaluationError` - 評価エラー
    ///
    /// # Raises
    /// - `EvaluationError::VariableNotFound` - 変数が見つからない場合
    pub fn get(&self, name: &str, line: usize) -> Result<VariableDeclarationNode, EvaluationError> {
        match self.map.get(name) {
            Some(value) => Ok(value.clone()),
            None => Err(EvaluationError::VariableNotFound {
                name: name.to_string(),
                line,
            }),
        }
    }

    /// 変数を設定する
    ///
    /// # Arguments
    /// - `definition` - 変数の定義
    ///
    /// # Returns
    /// - `Result<(), EvaluationError>` - 設定結果
    ///   - `()` - 正常終了
    ///   - `EvaluationError` - 評価エラー
    ///
    /// # Raises
    /// - `EvaluationError::ReassignmentError` - 変数の再代入が行われた場合
    pub fn set(
        &mut self,
        line: usize,
        definition: VariableDeclarationNode,
    ) -> Result<(), EvaluationError> {
        // 再代入を許可しない
        // let x = 1; x = 2;  # Error
        if self.map.contains_key(&definition.name) {
            return Err(EvaluationError::ReassignmentError {
                name: definition.name.clone(),
                line,
            });
        };

        self.map.insert(definition.name.clone(), definition);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::virtual_machine::ast::{
        ExpressionNode, LiteralNode, LiteralValue, Type, VariableDeclarationNode,
    };

    /// 変数マッパーの新規作成テスト
    /// 変数マッパーを新規作成し、正常に初期化されていることを確認します。
    ///
    /// let x: int = 1;
    #[test]
    fn test_variable_mapper_set_and_get() {
        let mut mapper = VariableMapper::new();

        // 変数宣言ノードを作成
        let variable_node = VariableDeclarationNode {
            name: "x".to_string(),
            var_type: Type::Integer,
            value: Box::new(ExpressionNode::Literal(Box::new(LiteralNode {
                value: LiteralValue::Integer(1),
            }))),
        };

        // 変数を設定
        let result: Result<(), EvaluationError> = mapper.set(1, variable_node.clone());

        // 正常に設定されたことを確認
        assert!(result.is_ok());

        // 設定した変数を取得
        let retrieved: Result<VariableDeclarationNode, EvaluationError> = mapper.get("x", 2);

        // 正しい値が取得できることを確認
        assert!(retrieved.is_ok());
        assert_eq!(retrieved.unwrap(), variable_node);
    }

    /// 存在しない変数を取得した場合のテスト
    /// 存在しない変数を取得した場合、エラーが返されることを確認します。
    ///
    /// print(y);  # Error
    #[test]
    fn test_variable_mapper_get_nonexistent_variable() {
        let mapper = VariableMapper::new();

        // 存在しない変数を取得
        let retrieved = mapper.get("y", 10);

        // エラーが返されることを確認
        assert!(retrieved.is_err());
    }

    /// 変数の再定義が行われた場合のテスト
    /// 変数の再定義が行われた場合、エラーが返されることを確認します。
    ///
    /// let x: int = 1;
    /// let x: int = 2;  # Error
    #[test]
    fn test_variable_mapper_reassignment_error() {
        let mut mapper = VariableMapper::new();

        // 最初の変数宣言ノード
        let first_variable_node = VariableDeclarationNode {
            name: "x".to_string(),
            var_type: Type::Integer,
            value: Box::new(ExpressionNode::Literal(Box::new(LiteralNode {
                value: LiteralValue::Integer(1),
            }))),
        };

        // 上書きする変数宣言ノード
        let second_variable_node = VariableDeclarationNode {
            name: "x".to_string(),
            var_type: Type::Integer,
            value: Box::new(ExpressionNode::Literal(Box::new(LiteralNode {
                value: LiteralValue::Integer(2),
            }))),
        };

        // 最初の変数を設定
        let first_result: Result<(), EvaluationError> = mapper.set(3, first_variable_node.clone());

        // 正常に設定されたことを確認
        assert!(first_result.is_ok());

        // 同じ名前で変数を再設定
        let second_result: Result<(), EvaluationError> =
            mapper.set(5, second_variable_node.clone());

        // エラーが返されることを確認
        assert!(second_result.is_err());
    }
}
