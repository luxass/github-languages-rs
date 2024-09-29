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
    fn get_all_languages() {
        let languages = get_languages();
        assert!(!languages.all_languages().is_empty());
    }

    #[test]
    fn get_language_by_name() {
        let languages = get_languages();
        let language = languages.get_by_name("Rust").unwrap();
        assert_eq!(language.name, "Rust");
    }

    #[test]
    fn no_language_with_name() {
        let languages = get_languages();
        let language = languages.get_by_name("Rusttt");
        assert!(language.is_none());
    }

    #[test]
    fn get_languages_by_extension() {
        let languages = get_languages();
        let extension_languages = languages.get_by_extension(".rs");
        assert!(!extension_languages.is_empty());
        assert!(extension_languages.iter().any(|l| l.name == "Rust"));
    }

    #[test]
    fn no_languages_with_extension() {
        let languages = get_languages();
        let extension_languages = languages.get_by_extension(".helloworld");
        assert!(extension_languages.is_empty());
    }

    #[test]
    fn get_languages_by_type() {
        let languages = get_languages();
        let programming_languages = languages.get_by_type("programming");
        assert!(!programming_languages.is_empty());
        assert!(programming_languages.iter().any(|l| l.name == "Rust"));
    }

    #[test]
    fn no_languages_with_type() {
        let languages = get_languages();
        let invalid_languages = languages.get_by_type("helloworld");
        assert!(invalid_languages.is_empty());
    }

    #[test]
    fn get_languages_by_alias() {
        let languages = get_languages();
        let alias_languages = languages.get_by_alias("rs");
        assert!(!alias_languages.is_empty());
        assert!(alias_languages.iter().any(|l| l.name == "Rust"));
    }

    #[test]
    fn no_languages_with_alias() {
        let languages = get_languages();
        let invalid_languages = languages.get_by_alias("helloworld");
        assert!(invalid_languages.is_empty());
    }

    #[test]
    fn get_languages_by_interpreter() {
        let languages = get_languages();
        let interpreter_languages = languages.get_by_interpreter("rust-script");
        assert!(!interpreter_languages.is_empty());
        assert!(interpreter_languages.iter().any(|l| l.name == "Rust"));
    }

    #[test]
    fn no_languages_with_interpreter() {
        let languages = get_languages();
        let invalid_languages = languages.get_by_interpreter("helloworld");
        assert!(invalid_languages.is_empty());
    }

    #[test]
    fn get_languages_by_filename() {
        let languages = get_languages();
        let languages_with_filenames = languages.get_by_filename("Cargo.lock");
        assert!(!languages_with_filenames.is_empty());
        assert!(languages_with_filenames.iter().any(|l| l.name == "TOML"));
    }

    #[test]
    fn no_languages_with_filename() {
        let languages = get_languages();
        let invalid_languages = languages.get_by_filename("helloworld");
        assert!(invalid_languages.is_empty());
    }

    #[test]
    fn should_get_language_by_id() {
        let languages = get_languages();
        let language = languages.get_by_id(100);
        assert!(language.is_some());
        assert_eq!(language.unwrap().name, "Elixir");
    }

    #[test]
    fn no_language_with_id() {
        let languages = get_languages();
        let language = languages.get_by_id(9999999);
        assert!(language.is_none());
    }

    #[test]
    fn language_info_structs() {
        let javascript = languages::JavaScript::info();
        assert_eq!(javascript.name, "JavaScript");
        assert_eq!(javascript.r#type, "programming");

        let typescript = languages::TypeScript::info();
        assert_eq!(typescript.name, "TypeScript");
        assert_eq!(typescript.r#type, "programming");

        let rust = languages::Rust::info();
        assert_eq!(rust.name, "Rust");
        assert_eq!(rust.r#type, "programming");

        let java = languages::Java::info();
        assert_eq!(java.name, "Java");
        assert_eq!(java.r#type, "programming");

        let python = languages::Python::info();
        assert_eq!(python.name, "Python");
        assert_eq!(python.r#type, "programming");
    }
}
