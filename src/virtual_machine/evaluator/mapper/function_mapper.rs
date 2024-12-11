use crate::virtual_machine::ast::FunctionDeclarationNode;
use crate::virtual_machine::evaluator::evaluation_error::EvaluationError;
use std::collections::HashMap;

pub(crate) struct FunctionMapper {
    map: HashMap<String, FunctionDeclarationNode>,
}

impl FunctionMapper {
    pub fn new() -> Self {
        FunctionMapper {
            map: Default::default(),
        }
    }

    /// 関数を取得する
    ///
    /// # Arguments
    /// - `name` - 関数名
    /// - `line` - 行番号
    ///
    /// # Returns
    /// - `Result<FunctionDeclarationNode, EvaluationError>` - Map結果
    ///   - `FunctionDeclarationNode` - 関数の定義
    ///   - `EvaluationError` - 評価エラー
    ///
    /// # Raises
    /// - `EvaluationError::FunctionNotFound` - 関数が見つからない場合
    pub fn get(&self, name: &str, line: usize) -> Result<FunctionDeclarationNode, EvaluationError> {
        match self.map.get(name) {
            Some(value) => Ok(value.clone()),
            None => Err(EvaluationError::FunctionNotFound {
                name: name.to_string(),
                line,
            }),
        }
    }

    /// 関数を設定する
    ///
    /// # Arguments
    /// - `name` - 関数名
    /// - `definition` - 関数の定義
    ///
    /// # Returns
    /// - `()` - 正常終了
    ///
    /// # Raises
    /// - `EvaluationError::ReassignmentError` - 関数の再定義が行われた場合
    pub fn set(
        &mut self,
        line: usize,
        definition: FunctionDeclarationNode,
    ) -> Result<(), EvaluationError> {
        let name: String = definition.name.clone();
        if self.map.contains_key(&name) {
            return Err(EvaluationError::ReassignmentError {
                name: name.clone(),
                line,
            });
        }

        self.map.insert(name, definition);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::virtual_machine::ast::{FunctionDeclarationNode, Type};

    /// 関数マッパーの新規作成テスト
    /// 関数マッパーを新規作成し、正常に初期化されていることを確認します。
    ///
    /// let x: fn = f(): void { return none; };
    /// f();  # OK
    #[test]
    fn test_function_mapper_set_and_get() {
        let mut mapper = FunctionMapper::new();

        // 関数宣言ノードを作成
        let function_node = FunctionDeclarationNode {
            name: "add".to_string(),
            params: vec![
                ("a".to_string(), Type::Integer),
                ("b".to_string(), Type::Integer),
            ],
            return_type: Type::Integer,
            body: vec![], // 実際のコードの場合は、関数の本体を記述するが、ここでは省略
        };

        // 関数を設定
        let result: Result<(), EvaluationError> = mapper.set(1, function_node.clone());

        // 正常に設定されたことを確認
        assert!(result.is_ok());

        // 設定した関数を取得
        let retrieved: Result<FunctionDeclarationNode, EvaluationError> = mapper.get("add", 2);

        // 正しい値が取得できることを確認
        assert!(retrieved.is_ok());
        assert_eq!(retrieved.unwrap(), function_node);
    }

    /// 存在しない関数を取得した場合のテスト
    /// 存在しない関数を取得した場合、エラーが返されることを確認します。
    ///
    /// let x: fn = f(): void { return none; };
    /// y();  # Error
    #[test]
    fn test_function_mapper_get_nonexistent_function() {
        let mapper = FunctionMapper::new();

        // 存在しない関数を取得
        let retrieved = mapper.get("nonexistent", 10);

        // エラーが返されることを確認
        assert!(retrieved.is_err());
    }

    /// 関数の再定義が行われた場合のテスト
    /// 関数の再定義が行われた場合、エラーが返されることを確認します。
    ///
    /// let x: fn = f(): void { return none; };
    /// x = f(): void { return none; };  # Error
    #[test]
    fn test_function_mapper_reassignment_error() {
        let mut mapper = FunctionMapper::new();

        // 最初の関数宣言ノード
        let first_function_node = FunctionDeclarationNode {
            name: "duplicate".to_string(),
            params: vec![],
            return_type: Type::Void,
            body: vec![], // 実際のコードの場合は、関数の本体を記述するが、ここでは省略
        };

        // 上書きする関数宣言ノード
        let second_function_node = FunctionDeclarationNode {
            name: "duplicate".to_string(),
            params: vec![],
            return_type: Type::Void,
            body: vec![], // 実際のコードの場合は、関数の本体を記述するが、ここでは省略
        };

        // 最初の関数を設定
        let first_result: Result<(), EvaluationError> = mapper.set(3, first_function_node);

        // 正常に設定されたことを確認
        assert!(first_result.is_ok());

        // 同じ名前で関数を再設定
        let second_result: Result<(), EvaluationError> = mapper.set(5, second_function_node);

        // エラーが返されることを確認
        assert!(second_result.is_err());
    }
}
