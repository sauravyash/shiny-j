use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum SymbolType {
    Variable,
    Function,
    ReactiveValue,
    ReactiveExpression,
    UIComponent,
    Module,
    Output,
}

#[derive(Debug, Clone)]
pub struct SymbolInfo {
    pub name: String,
    pub symbol_type: SymbolType,
    pub scope: String,
    pub data_type: Option<String>,
    pub is_mutable: bool,
    pub dependencies: Vec<String>,
}

pub struct SymbolTable {
    symbols: HashMap<String, SymbolInfo>,
    scopes: Vec<String>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            symbols: HashMap::new(),
            scopes: vec!["global".to_string()],
        }
    }

    pub fn enter_scope(&mut self, scope_name: &str) {
        self.scopes.push(scope_name.to_string());
    }

    pub fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn add_symbol(
        &mut self,
        name: &str,
        symbol_type: SymbolType,
        data_type: Option<String>,
        is_mutable: bool,
    ) {
        let current_scope = self.scopes.last().unwrap().clone();
        let symbol_info = SymbolInfo {
            name: name.to_string(),
            symbol_type,
            scope: current_scope,
            data_type,
            is_mutable,
            dependencies: Vec::new(),
        };
        self.symbols.insert(name.to_string(), symbol_info);
    }

    pub fn get_symbol(&self, name: &str) -> Option<&SymbolInfo> {
        self.symbols.get(name)
    }

    pub fn add_dependency(&mut self, symbol_name: &str, dependency: &str) {
        if let Some(symbol) = self.symbols.get_mut(symbol_name) {
            symbol.dependencies.push(dependency.to_string());
        }
    }

    pub fn get_symbols_in_scope(&self, scope: &str) -> Vec<&SymbolInfo> {
        self.symbols.values().filter(|s| s.scope == scope).collect()
    }

    pub fn is_reactive(&self, name: &str) -> bool {
        match self.get_symbol(name) {
            Some(info) => matches!(
                info.symbol_type,
                SymbolType::ReactiveValue | SymbolType::ReactiveExpression
            ),
            None => false,
        }
    }
}
