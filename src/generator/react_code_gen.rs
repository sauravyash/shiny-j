use crate::transformer::ReactAST;
use crate::utils::TranspilerError;

pub fn generate_react_code(react_ast: &ReactAST) -> Result<String, TranspilerError> {
    let mut output = String::new();

    // Generate imports
    output.push_str(&generate_imports(&react_ast.imports));
    output.push_str("\n\n");

    // Generate components
    for component in &react_ast.components {
        output.push_str(&generate_component(component));
        output.push_str("\n\n");
    }

    // Generate main App component
    output.push_str(&generate_main_app(&react_ast));

    Ok(output)
}

fn generate_imports(imports: &[String]) -> String {
    imports.join("\n")
}

fn generate_component(component: &ReactComponent) -> String {
    format!(
        "function {}({}) {{\n  {}\n}}",
        component.name,
        component.props.join(", "),
        component.body
    )
}

fn generate_main_app(ast: &ReactAST) -> String {
    let component_names = ast
        .components
        .iter()
        .map(|c| c.name.clone())
        .collect::<Vec<_>>()
        .join(", ");

    format!(
        "function App() {{\n  return (\n    <div>\n      {}\n    </div>\n  );\n}}\n\nexport default App;",
        ast.components
            .iter()
            .map(|c| format!("<{} />", c.name))
            .collect::<Vec<_>>()
            .join("\n      ")
    )
}
