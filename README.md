# Bucceloang Interpreter in Rust

<p align="center">
  <img src="assets/logo.png" alt="Bucceloang Logo" width="200"/>
</p>

**Bucceloang** is a interpreter written in Rust, inspired by the book [_Crafting Interpreters_](https://craftinginterpreters.com/).

**Status:** ⚠️ _In development!_

## 🧠 Language Features

- **Dynamically Typed** – No type annotations, types are checked at runtime.
- **Garbage Collected** – Automatic memory management.
- **Lexical Scoping** – Block-level variable visibility with closures.
- **First-Class Functions** – Functions are values; supports closures.
- **Built-in Types** – `number`, `string`, `boolean`, and `nil`.

## ▶️ Usage

Run the interactive REPL:

```sh
cargo run
```

Execute a script:

```sh
cargo run -- path/to/file.bl
```

## ⚙️ Build Instructions

No external dependencies required. Just run:

```sh
cargo build
```

Rust stable is sufficient.

## 📂 Project Documentation

Generate and open the project documentation with:

```sh
cargo doc --open
```

## 📖 Resources Used

- 📘 [_Crafting Interpreters_ – Robert Nystrom](https://craftinginterpreters.com/)
