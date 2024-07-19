mod transformer;

#[derive(Debug)]
pub struct ReactAST {
    // Define the structure of your React AST here
    // For example:
    pub components: Vec<ReactComponent>,
    pub imports: Vec<String>,
    // Add other fields as needed
}

#[derive(Debug)]
pub struct ReactComponent {
    pub name: String,
    pub props: Vec<String>,
    pub body: String,
    // Add other fields as needed
}

pub fn transform_to_react_ast(
    analyzed_ast: &crate::analyzer::AnalyzedShinyAST,
) -> Result<ReactAST, crate::utils::TranspilerError> {
    transformer::transform_to_react_ast(analyzed_ast)
}

// You might want to implement some methods on ReactAST
impl ReactAST {
    pub fn new() -> Self {
        ReactAST {
            components: Vec::new(),
            imports: Vec::new(),
        }
    }

    // Add other methods as needed
}
