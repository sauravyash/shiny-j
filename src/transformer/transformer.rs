mod transformer;

pub use self::transformer::ReactAST;

pub fn transform_to_react_ast(
    analyzed_ast: &crate::analyzer::AnalyzedShinyAST,
) -> Result<ReactAST, crate::utils::TranspilerError> {
    ast_transformer::transform_to_react_ast(analyzed_ast)
}
