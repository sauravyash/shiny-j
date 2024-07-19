pub(crate) mod react_code_gen;

pub fn generate_react_code(
    react_ast: &crate::transformer::ReactAST,
) -> Result<String, crate::utils::TranspilerError> {
    generate_react_code(react_ast)
}
