use shiny_to_react_transpiler::analyzer::analyze_shiny_ast;
use shiny_to_react_transpiler::parser::ShinyAST;

#[test]
fn test_analyze_reactive_values() {
    let ast = ShinyAST { /* populate with test data */ };
    let analyzed_ast = analyze_shiny_ast(&ast).unwrap();
    assert_eq!(analyzed_ast.reactive_values.len(), 1);
    assert_eq!(analyzed_ast.reactive_values[0].name, "bins");
}
