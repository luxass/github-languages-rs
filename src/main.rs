//! All GitHub's supported languages.
//!
//! [Linguist]: https://github.com/github-linguist/linguist

use github_languages::{
    get_languages,
    languages::{
        Java,
        JavaScript,
        TypeScript
    }
};


fn main() {
    println!("{:?}", JavaScript::info());
    println!("{:?}", TypeScript::info());
    println!("{:?}", Java::info());
    get_languages().all_languages().iter().for_each(|lang| {
        println!("{:?}", lang.color);
    });
}
