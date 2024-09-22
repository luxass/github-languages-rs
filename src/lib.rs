//! All GitHub's supported languages.
//!
//! [Linguist]: https://github.com/github-linguist/linguist

mod generated;

pub use generated::{
    get_languages, Languages, Language
};

pub mod languages {
  pub use super::generated::*;
}






