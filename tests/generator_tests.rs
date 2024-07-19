use shiny_to_react_transpiler::generator::generate_react_code;
use shiny_to_react_transpiler::transformer::ReactAST;

#[test]
fn test_generate_component() {
    let react_ast = ReactAST { /* populate with test data */ };
    let result = generate_react_code(&react_ast).unwrap();
    assert!(result.contains("function App()"));
    assert!(result.contains("return ("));
}
