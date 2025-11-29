# Rust-Hangman-Game
# Overview

I'm a computer engineering student learning different programming languages. For this project, I built a Hangman game in Rust to help me learn this new language.

The game lets players guess programming language names letter by letter. It includes features like a word bank, hints, and ASCII art that shows how many wrong guesses you've made. I built this to understand how Rust works and to practice important programming concepts.

This project helped me learn Rust's syntax and how it handles data structures and object-oriented programming. Even though Rust is known for being complex, I was able to create a fun, working game that demonstrates the language's capabilities.

[Software Demo Video]()

# Development Environment

I developed this software using Visual Studio Code with the Rust Analyzer extension for language support and debugging capabilities. The project was built using Rust's native toolchain, including Cargo for package management.

The program is written entirely in Rust using only the standard library, leveraging several key modules:
- `std::io` for terminal input/output operations and error handling
- `std::collections::HashMap` for word bank storage and retrieval
- Rust's ownership system for memory-safe string and vector management
- Pattern matching and error handling throughout the game logic

# Useful Websites

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Standard Library Documentation](https://doc.rust-lang.org/std/)
- [Rustlings Course](https://github.com/rust-lang/rustlings)

# Future Work

- Implement true random word selection from the entire word bank instead of hardcoding a specific word
- Add multiple difficulty levels with varying word lengths and guess limits
- Add comprehensive unit tests for core game functions including guess validation and win/lose conditions
- Extend the game to support multi-word phrases and categories beyond programming languages
