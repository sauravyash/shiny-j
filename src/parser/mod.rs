mod ast;
mod lexer;

pub use self::ast::ShinyAST;

pub fn parse_shiny_app(code: &str) -> Result<ShinyAST, crate::utils::TranspilerError> {
    // Implementation here
}
