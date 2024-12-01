use std::collections::HashMap;
use crate::virtual_machine::ast::VariableDeclarationNode;

pub(crate) struct VariableMapper {
    function_map: HashMap<String, VariableDeclarationNode>
}

impl VariableMapper {
    pub fn new() -> Self {
        VariableMapper {
            function_map: Default::default(),
        }
    }

    pub fn get(&self, name: &str) -> Option<VariableDeclarationNode> {
        self.function_map.get(name).cloned()
    }

    pub fn set(&mut self, name: String, definition: VariableDeclarationNode) {
        self.function_map.insert(name, definition);
    }
}
