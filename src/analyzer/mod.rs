mod semantic_analyzer;

pub use self::semantic_analyzer::AnalyzedShinyAST;
mod symbol_table;

pub use semantic_analyzer::analyze_shiny_ast;
pub use symbol_table::SymbolTable;
