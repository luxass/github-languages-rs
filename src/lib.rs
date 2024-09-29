//! All GitHub's supported languages.
//!
//! [Linguist]: https://github.com/github-linguist/linguist
//!
//! # Example
//! ```rust
//! use github_languages::get_languages;
//!
//! let languages = get_languages();
//!
//! for language in languages.all_languages() {
//!    println!("{:?}", language);
//! }
//! ```

#[rustfmt::skip]
mod generated;

pub use generated::{get_languages, Language, Languages, LANGUAGES};

pub mod languages {
    pub use super::generated::*;
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_get_languages() {
        let languages = get_languages();
        assert!(languages.all_languages().len() > 0);
    }

    #[test]
    fn test_get_language_by_name() {
        let languages = get_languages();
        let language = languages.get_by_name("Rust").unwrap();
        assert_eq!(language.name, "Rust");
    }

    #[test]
    fn test_get_languages_by_extension() {
        let languages = get_languages();
        let extension_languages = languages.get_by_extension(".rs");
        assert!(extension_languages.len() > 0);
        assert!(extension_languages.iter().any(|l| l.name == "Rust"));
    }

    #[test]
    fn test_language_structs() {
        let javascript = languages::JavaScript::info();
        assert_eq!(javascript.name, "JavaScript");
        assert_eq!(javascript.type_, "programming");

        let typescript = languages::TypeScript::info();
        assert_eq!(typescript.name, "TypeScript");
        assert_eq!(typescript.type_, "programming");

        let rust = languages::Rust::info();
        assert_eq!(rust.name, "Rust");
        assert_eq!(rust.type_, "programming");

        let java = languages::Java::info();
        assert_eq!(java.name, "Java");
        assert_eq!(java.type_, "programming");

        let python = languages::Python::info();
        assert_eq!(python.name, "Python");
        assert_eq!(python.type_, "programming");
    }
}
