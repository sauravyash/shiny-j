use shiny_to_react_transpiler::parser::parse_shiny_app;

#[test]
fn test_parse_simple_ui() {
    let shiny_code = r#"ui <- fluidPage(titlePanel("Test"))"#;
    let ast = parse_shiny_app(shiny_code).unwrap();
    assert_eq!(ast.ui_elements.len(), 1);
    assert_eq!(ast.ui_elements[0].element_type, "fluidPage");
}
