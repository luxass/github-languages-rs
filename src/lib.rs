//! All GitHub's supported languages.
//!
//! [Linguist]: https://github.com/github-linguist/linguist
//!
//! # Example
//! ```
//! use github_languages::languages;
//!
//! let languages = languages::get_languages();
//!
//! for language in languages {
//!    println!("{:?}", language);
//! }
//! ```

#[rustfmt::skip]
mod generated;

pub use generated::{get_languages, Language, Languages, LANGUAGES};

pub mod languages {
    pub use super::generated::*;
}
