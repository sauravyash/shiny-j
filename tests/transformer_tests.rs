use shiny_to_react_transpiler::analyzer::AnalyzedShinyAST;
use shiny_to_react_transpiler::transformer::transform_to_react_ast;

#[test]
fn test_transform_ui_element() {
    let analyzed_ast = AnalyzedShinyAST { /* populate with test data */ };
    let react_ast = transform_to_react_ast(&analyzed_ast).unwrap();
    assert_eq!(react_ast.components.len(), 1);
    assert_eq!(react_ast.components[0].name, "App");
}
