use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum TranspilerError {
    ParseError(String),
    AnalysisError(String),
    TransformationError(String),
    CodeGenerationError(String),
    // Add more error types as needed
}

impl fmt::Display for TranspilerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TranspilerError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            TranspilerError::AnalysisError(msg) => write!(f, "Analysis error: {}", msg),
            TranspilerError::TransformationError(msg) => write!(f, "Transformation error: {}", msg),
            TranspilerError::CodeGenerationError(msg) => {
                write!(f, "Code generation error: {}", msg)
            }
        }
    }
}

impl Error for TranspilerError {}
