//! All GitHub's supported languages.
//!
//! [Linguist]: https://github.com/github-linguist/linguist

#[rustfmt::skip]
mod generated;

pub use generated::{get_languages, Language, Languages, LANGUAGES};

pub mod languages {
    pub use super::generated::*;
}
