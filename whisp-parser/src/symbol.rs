use std::collections::HashMap;

#[derive(Debug)]
pub struct SymbolInfo;

#[derive(Debug)]
pub struct SymbolTable {
    stack: Vec<HashMap<String, SymbolInfo>>
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            stack: vec![HashMap::new()],
        }
    }

    pub fn enter_scope(&mut self) {
        self.stack.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        self.stack.pop();
    }

    pub fn define(&mut self, name: String, info: SymbolInfo) {
        if let Some(current) = self.stack.last_mut() {
            current.insert(name.clone(), info);
        }
    }

    pub fn resolve(&self, name: &str) -> Option<&SymbolInfo> {
        for scope in self.stack.iter().rev() {
            if let Some(info) = scope.get(name) {
                return Some(info);
            }
        }
        None
    }
}
