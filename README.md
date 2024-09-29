<h1 align="center">GitHub Languages</h1>
<div align="center">
  <strong>
    All GitHub's supported languages.
  </strong>
</div>
<br />

<div align="center">
  <a href="https://img.shields.io/github/actions/workflow/status/luxass/github-languages-rs/ci.yaml?query=branch%3Amain">
    <img src="https://img.shields.io/github/actions/workflow/status/luxass/github-languages-rs/ci.yaml?branch=main&style=flat-square" alt="actions status" /></a>
  <a href="https://crates.io/crates/github-languages">
    <img src="https://img.shields.io/crates/v/github-languages.svg?style=flat-square"
    alt="Crates.io version" /></a>
  <a href="https://docs.rs/github-languages">
  <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>
  <a href="https://crates.io/crates/github-languages">
    <img src="https://img.shields.io/crates/d/github-languages.svg?style=flat-square" alt="Download" />
  </a>
</div>

<div align="center">
  <h4>
    <a href="#install">
      Install
    </a>
    <span> | </span>
    <a href="#usage">
      Usage
    </a>
    <span> | </span>
    <a href="https://docs.rs/github-languages">
      Docs
    </a>
  </h4>
</div>

<br />

## Install

Add the following to your `Cargo.toml`:

```toml
[dependencies]
github-languages = "0.1"
```

## Usage

### Accessing Individual Language Information

You can access information about a specific language directly using its struct:

```rust
use github_languages::{Rust, Python};

fn main() {
    let rust_info = Rust::info();
    println!("Rust color: {}", rust_info.color);

    let python_info = Python::info();
    println!("Python ace_mode: {}", python_info.ace_mode);
}
```

### Dynamic Language Lookups

For dynamic lookups, use the `LANGUAGES` static variable:

```rust
use github_languages::LANGUAGES;

fn main() {
    // Lookup by name
    if let Some(rust_lang) = LANGUAGES.get_by_name("Rust") {
        println!("Rust color: {}", rust_lang.color);
    }
    
    // Lookup by file extension
    if let Some(py_lang) = LANGUAGES.get_by_extension(".py") {
        println!("Python ace_mode: {}", py_lang.ace_mode);
    }
    
    // Iterate over all languages
    for lang in LANGUAGES.all_languages() {
        println!("Language: {}, ID: {}", lang.name, lang.language_id);
    }
}
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License
