use std::env;
use std::fs;
use std::path::Path;
use std::process;

// Import modules from your lib.rs
use shiny_to_react_transpiler::{parser, analyzer, transformer, generator};

fn main() {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <path_to_shiny_app>", args[0]);
        process::exit(1);
    }

    let input_path = &args[1];

    // Read the input file
    let shiny_code = match fs::read_to_string(input_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file {}: {}", input_path, e);
            process::exit(1);
        }
    };

    // Perform the transpilation
    match transpile(&shiny_code) {
        Ok(react_code) => {
            // Write the output to a file
            let output_path = Path::new(input_path).with_extension("jsx");
            if let Err(e) = fs::write(&output_path, react_code) {
                eprintln!("Error writing output file: {}", e);
                process::exit(1);
            }
            println!("Transpilation successful. Output written to {:?}", output_path);
        }
        Err(e) => {
            eprintln!("Transpilation failed: {}", e);
            process::exit(1);
        }
    }
}

fn transpile(shiny_code: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Step 1: Parse Shiny code into AST
    let shiny_ast = parser::parse_shiny_app(shiny_code)?;

    // Step 2: Analyze the Shiny AST
    let analyzed_ast = analyzer::analyze_shiny_ast(&shiny_ast)?;

    // Step 3: Transform Shiny AST to React AST
    let react_ast = transformer::transform_to_react_ast(&analyzed_ast)?;

    // Step 4: Generate React code
    let react_code = generator::generate_react_code(&react_ast)?;

    Ok(react_code)
}