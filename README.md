# Bucceloang - Interpreter in Rust

<p align="center">
  <img src="assets/icon.png" alt="Bucceloang Logo" width="200"/>
</p>

**Bucceloang** is an interpreter written in Rust that combines simplicity with powerful programming constructs, inspired by the book [_Crafting Interpreters_](https://craftinginterpreters.com/). It features a sophisticated execution pipeline with lexing, parsing, static analysis, and interpretation phases.

> [!IMPORTANT] 
> _In Beta!_

## 🧠 Language Features

### Core Features

- **Dynamically Typed** – No type annotations, types are checked at runtime
- **Garbage Collected** – Automatic memory management using Rust's reference counting
- **Lexical Scoping** – Block-level variable visibility with closures
- **First-Class Functions** – Functions are values; supports closures
- **Built-in Types** – `number` (64-bit float), `string` (UTF-8), `boolean`, `nil`, `function`, and `class`

### Operators

- **Arithmetic**: `+` (also string concatenation), `-`, `*`, `/`
- **Comparison**: `==`, `!=`, `<`, `<=`, `>`, `>=`
- **Logical**: `&&`, `||`, `!`

### Syntax Highlights

- Semicolon-terminated statements
- Block-scoped with curly braces
- C-style comments (`//` and `/* */`)
- `var` for variable declarations
- `fn` for function definitions
- `class` for class declarations

## 🔄 Execution Model

Bucceolang processes code through several sophisticated phases:

1. **Scanning** – Breaks source code into tokens, handles comments and tracks line numbers
2. **Parsing** – Builds Abstract Syntax Tree (AST) with operator precedence
3. **Static Analysis** – Resolves variables, validates scopes, and performs semantic checks
4. **Interpretation** – Executes the program with automatic memory management

## ▶️ Usage

### Command Line Interface

Run the interactive REPL:

```sh
cargo run
```

Execute a script:

```sh
cargo run -- path/to/file.bl
```

### Web Interface

Bucceolang can also be run directly in the browser through WebAssembly compilation, making it accessible for web-based applications.

## ⚙️ Build Instructions

### Local Build

No external dependencies required except Rust stable. Just run:

```sh
cargo build
```

### WebAssembly Build

The project includes WebAssembly support through `wasm-bindgen`. Build the web version with:

```sh
wasm-pack build
```

## 📂 Project Documentation

Generate and open the project documentation with:

```sh
cargo doc --open
```

For comprehensive language documentation, see `assets/documentation.pdf`.

## 📖 Resources Used

- 📘 [_Crafting Interpreters_ – Robert Nystrom](https://craftinginterpreters.com/)
