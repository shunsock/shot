use crate::virtual_machine::ast::FunctionDeclarationNode;
use std::collections::HashMap;

pub(crate) struct FunctionMapper {
    function_map: HashMap<String, FunctionDeclarationNode>,
}

impl FunctionMapper {
    pub fn new() -> Self {
        FunctionMapper {
            function_map: Default::default(),
        }
    }

    pub fn get(&self, name: &str) -> Option<FunctionDeclarationNode> {
        self.function_map.get(name).cloned()
    }

    pub fn set(&mut self, name: String, definition: FunctionDeclarationNode) {
        self.function_map.insert(name, definition);
    }
}
