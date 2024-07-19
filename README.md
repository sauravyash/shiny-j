# Shiny to React Transpiler

## Overview

The Shiny to React Transpiler is a tool designed to convert R Shiny applications into React.js applications. This project aims to bridge the gap between R-based web applications and modern JavaScript frameworks, allowing developers to leverage the power of React while maintaining the functionality of their Shiny apps.

## Features

- Parse R Shiny code into an Abstract Syntax Tree (AST)
- Analyze and transform the Shiny AST into a React-compatible structure
- Generate equivalent React code from the transformed AST
- Maintain Shiny's reactivity model using React hooks
- Support for core Shiny UI components and layouts

## Getting Started

### Prerequisites

- Rust (latest stable version)

### Installation

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/shiny-to-react-transpiler.git
   cd shiny-to-react-transpiler
   ```

2. Build the project:
   ```
   cargo build --release
   ```

### Usage

To transpile a Shiny app, run:

```
cargo run -- path/to/your/shiny/app.R
```

The transpiled React code will be output to `./output/` by default.

## Project Structure

- `src/`: Source code for the transpiler
  - `parser/`: R code parsing module
  - `analyzer/`: Shiny AST analysis module
  - `transformer/`: AST transformation module
  - `generator/`: React code generation module
- `tests/`: Integration and unit tests
- `examples/`: Sample Shiny applications for testing

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- The R and Shiny communities for their excellent work
- The Rust community for providing powerful tools for systems programming

# Shiny to React Transpiler: Component Status

| Component | Status | Tests |
|-----------|--------|-------|
| Parser | In Progress | Partially Implemented |
| Analyzer | Not Started | Not Implemented |
| Transformer | Not Started | Not Implemented |
| Generator | Not Started | Not Implemented |
| SymbolTable | Finished | Passing |
| UI Component Mapping | Not Started | Not Implemented |
| Reactivity Handling | Not Started | Not Implemented |
| Error Handling | In Progress | Partially Implemented |
| CLI Interface | Not Started | Not Implemented |
| Documentation | In Progress | N/A |

## Legend:
- Status: Not Started, In Progress, Finished
- Tests: Not Implemented, Failing, Partially Implemented, Passing, N/A (for components that don't require direct testing)

## Recent Updates:
- [Date]: Implemented basic structure for SymbolTable
- [Date]: Started work on the Parser component
- [Date]: Initialized project structure and documentation

## Next Steps:
1. Complete the Parser implementation
2. Begin work on the Analyzer component
3. Implement more comprehensive tests for existing components

Feel free to update this table as you make progress on your project. This will help you and any collaborators quickly understand the current state of the transpiler.

## Contact

If you have any questions or feedback, please open an issue on this GitHub repository.