use proc_macro2::{Ident, Literal, TokenStream};
use quote::quote;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;
use std::{env, fs};

#[derive(Debug, Deserialize)]
struct GitHubLanguageData {
    r#type: Option<String>,
    color: Option<String>,
    extensions: Option<Vec<String>>,
    aliases: Option<Vec<String>>,
    tm_scope: Option<String>,
    ace_mode: Option<String>,
    language_id: Option<u64>,
    codemirror_mode: Option<String>,
    codemirror_mime_type: Option<String>,
    wrap: Option<bool>,
    filenames: Option<Vec<String>>,
    group: Option<String>,
    interpreters: Option<Vec<String>>,
    fs_name: Option<String>,
    searchable: Option<bool>,
}

#[derive(Debug, Clone)]
struct LanguageData {
    name: String,
    r#type: String,
    color: String,
    extensions: Vec<String>,
    aliases: Vec<String>,
    tm_scope: String,
    ace_mode: String,
    language_id: u64,
    codemirror_mode: Option<String>,
    codemirror_mime_type: Option<String>,
    wrap: Option<bool>,
    filenames: Vec<String>,
    group: Option<String>,
    interpreters: Vec<String>,
    fs_name: Option<String>,
    searchable: Option<bool>,
}

fn main() {
    println!("cargo:rerun-if-changed=languages.yml");
    println!("cargo:rerun-if-changed=build.rs");
    let is_docs_rs = env::var("DOCS_RS").is_ok();
    let dest_path = Path::new("./src").join("generated.rs");

    let languages_data = fs::read_to_string("languages.yml").expect("Failed to read languages.yml");

    let github_languages: HashMap<String, GitHubLanguageData> =
        serde_yaml::from_str(&languages_data).expect("Failed to parse languages.yml");

    let languages = convert_github_format(github_languages);

    let generated_code = generate_language_code(&languages);

    // try to format with prettyplease, but fallback to direct formatting if it fails
    let formatted = match syn::parse2(generated_code.clone()) {
        Ok(syntax_tree) => format!(
            "#![allow(clippy::all)]\n{}",
            prettyplease::unparse(&syntax_tree)
        ),
        Err(_) => {
            // Fallback: just use the direct token stream with some basic formatting
            eprintln!(
                "Warning: Could not parse generated code for formatting, using basic formatting"
            );
            format!("{}", generated_code)
        }
    };

    if is_docs_rs {
        println!("cargo:warning=Generated code:\n{}", formatted);
    } else {
        fs::write(&dest_path, formatted.as_bytes()).expect("Failed to write generated.rs");
    }
}

fn convert_github_format(
    github_languages: HashMap<String, GitHubLanguageData>,
) -> Vec<LanguageData> {
    let mut languages = Vec::new();

    for (name, github_data) in github_languages {
        // skip if essential fields are missing
        if github_data.language_id.is_none() {
            eprintln!(
                "Warning: Skipping language '{}' - missing language_id",
                name
            );
            continue;
        }

        let language = LanguageData {
            name: name.clone(),
            r#type: github_data.r#type.unwrap_or_else(|| "data".to_string()),
            color: github_data.color.unwrap_or_else(|| "#000000".to_string()),
            extensions: github_data.extensions.unwrap_or_else(Vec::new),
            aliases: github_data.aliases.unwrap_or_else(Vec::new),
            tm_scope: github_data
                .tm_scope
                .unwrap_or_else(|| format!("source.{}", name.to_lowercase().replace(' ', "_"))),
            ace_mode: github_data.ace_mode.unwrap_or_else(|| "text".to_string()),
            language_id: github_data.language_id.unwrap(),
            codemirror_mode: github_data.codemirror_mode,
            codemirror_mime_type: github_data.codemirror_mime_type,
            wrap: github_data.wrap,
            filenames: github_data.filenames.unwrap_or_else(Vec::new),
            group: github_data.group,
            interpreters: github_data.interpreters.unwrap_or_else(Vec::new),
            fs_name: github_data.fs_name,
            searchable: github_data.searchable,
        };

        languages.push(language);
    }

    // sort by name for consistent output
    languages.sort_by(|a, b| a.name.cmp(&b.name));

    languages
}

fn generate_language_code(languages: &[LanguageData]) -> TokenStream {
    // all imports at the top
    let imports = quote! {
        use crate::define_languages;
        use phf::phf_map;
    };

    // generate the single macro call with all languages
    let all_languages_call = generate_all_languages_call(languages);

    // generate extension map separately since PHF maps can't be easily generated in macros
    let extension_map = generate_extension_map(languages);

    quote! {
        #imports

        #all_languages_call

        #extension_map
    }
}

fn generate_all_languages_call(languages: &[LanguageData]) -> TokenStream {
    let language_definitions = languages.iter().map(|lang| {
        let struct_name = sanitize_identifier_as_ident(&lang.name);
        let name = &lang.name;
        let lang_type = &lang.r#type;
        let color = &lang.color;
        let tm_scope = &lang.tm_scope;
        let ace_mode = &lang.ace_mode;
        let language_id = lang.language_id;

        let extensions = string_vec_to_token_array(&lang.extensions);
        let aliases = string_vec_to_token_array(&lang.aliases);
        let filenames = string_vec_to_token_array(&lang.filenames);
        let interpreters = string_vec_to_token_array(&lang.interpreters);

        // handle optional fields
        let codemirror_mode = match &lang.codemirror_mode {
            Some(mode) => quote! { codemirror_mode: #mode, },
            None => quote! {},
        };

        let codemirror_mime_type = match &lang.codemirror_mime_type {
            Some(mime) => quote! { codemirror_mime_type: #mime, },
            None => quote! {},
        };

        let wrap = match lang.wrap {
            Some(w) => quote! { wrap: #w, },
            None => quote! {},
        };

        let group = match &lang.group {
            Some(g) => quote! { group: #g, },
            None => quote! {},
        };

        let fs_name = match &lang.fs_name {
            Some(fs) => quote! { fs_name: #fs, },
            None => quote! {},
        };

        let searchable = match lang.searchable {
            Some(s) => quote! { searchable: #s, },
            None => quote! {},
        };

        quote! {
            #struct_name => {
                name: #name,
                r#type: #lang_type,
                color: #color,
                extensions: [#extensions],
                aliases: [#aliases],
                tm_scope: #tm_scope,
                ace_mode: #ace_mode,
                language_id: #language_id,
                #codemirror_mode
                #codemirror_mime_type
                #wrap
                filenames: [#filenames],
                #group
                interpreters: [#interpreters],
                #fs_name
                #searchable
            },
        }
    });

    quote! {
        define_languages! {
            #(
                #language_definitions
            )*
        }
    }
}

fn generate_extension_map(languages: &[LanguageData]) -> TokenStream {
    // generate by_extension map - this is more complex since multiple languages can share extensions
    let mut extension_map: HashMap<String, Vec<Ident>> = HashMap::new();
    for lang in languages {
        let struct_name = sanitize_identifier_as_ident(&lang.name);
        for ext in &lang.extensions {
            extension_map
                .entry(ext.clone())
                .or_default()
                .push(struct_name.clone());
        }
    }

    let mut by_extension_entries: Vec<_> = extension_map.iter().collect();
    by_extension_entries.sort_by_key(|(ext, _)| *ext);

    let by_extension_entries = by_extension_entries.iter().map(|(ext, structs)| {
        let struct_fns = structs.iter().map(|s| quote! { #s::info });
        quote! { #ext => &[ #( #struct_fns ),* ] }
    });

    quote! {
        // Override the placeholder extension map with the real one
        static BY_EXTENSION: phf::Map<&'static str, &'static [fn() -> LanguageInfo]> = phf_map! {
            #( #by_extension_entries ),*
        };
    }
}

fn string_vec_to_token_array(strings: &[String]) -> TokenStream {
    let literals = strings.iter().map(|s| Literal::string(s));
    quote! { #( #literals ),* }
}

fn sanitize_identifier_as_ident(name: &str) -> Ident {
    let sanitized = sanitize_identifier(name);
    Ident::new(&sanitized, proc_macro2::Span::call_site())
}

fn sanitize_identifier(name: &str) -> String {
    let mut struct_name = name.to_string();
    struct_name = struct_name.replace(" ", "_");
    struct_name = struct_name.replace("-", "_");
    struct_name = struct_name.replace("+", "p");
    struct_name = struct_name.replace("#", "sharp");
    struct_name = struct_name.replace("*", "star");
    struct_name = struct_name.replace("'", "");
    struct_name = struct_name.replace(".", "");
    struct_name = struct_name.replace("(", "");
    struct_name = struct_name.replace(")", "");
    struct_name = struct_name.replace("/", "_");
    struct_name = struct_name.replace(":", "_");

    struct_name = struct_name
        .split('_')
        .map(|s| {
            let mut s = s.chars();
            match s.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().chain(s).collect(),
            }
        })
        .collect::<Vec<_>>()
        .join("");

    if struct_name.starts_with(char::is_numeric) {
        struct_name = format!("_{}", struct_name);
    }

    // handle Rust keywords and common conflicts
    match struct_name.as_str() {
        "Self" => "_Self".to_string(),
        "Type" => "TypeLang".to_string(),
        "Struct" => "StructLang".to_string(),
        "Enum" => "EnumLang".to_string(),
        "Fn" => "FnLang".to_string(),
        "Let" => "LetLang".to_string(),
        "Mut" => "MutLang".to_string(),
        "Ref" => "RefLang".to_string(),
        "Match" => "MatchLang".to_string(),
        "If" => "IfLang".to_string(),
        "Else" => "ElseLang".to_string(),
        "For" => "ForLang".to_string(),
        "While" => "WhileLang".to_string(),
        "Loop" => "LoopLang".to_string(),
        "Break" => "BreakLang".to_string(),
        "Continue" => "ContinueLang".to_string(),
        "Return" => "ReturnLang".to_string(),
        "True" => "TrueLang".to_string(),
        "False" => "FalseLang".to_string(),
        "As" => "AsLang".to_string(),
        "Use" => "UseLang".to_string(),
        "Mod" => "ModLang".to_string(),
        "Pub" => "PubLang".to_string(),
        "Const" => "ConstLang".to_string(),
        "Static" => "StaticLang".to_string(),
        "Super" => "SuperLang".to_string(),
        "Extern" => "ExternLang".to_string(),
        "Crate" => "CrateLang".to_string(),
        "Move" => "MoveLang".to_string(),
        "Box" => "BoxLang".to_string(),
        "Impl" => "ImplLang".to_string(),
        "Trait" => "TraitLang".to_string(),
        "Where" => "WhereLang".to_string(),
        "Unsafe" => "UnsafeLang".to_string(),
        "Async" => "AsyncLang".to_string(),
        "Await" => "AwaitLang".to_string(),
        "Dyn" => "DynLang".to_string(),
        "Abstract" => "AbstractLang".to_string(),
        "Become" => "BecomeLang".to_string(),
        "Do" => "DoLang".to_string(),
        "Final" => "FinalLang".to_string(),
        "Macro" => "MacroLang".to_string(),
        "Override" => "OverrideLang".to_string(),
        "Priv" => "PrivLang".to_string(),
        "Try" => "TryLang".to_string(),
        "Union" => "UnionLang".to_string(),
        "Virtual" => "VirtualLang".to_string(),
        "Yield" => "YieldLang".to_string(),
        _ => struct_name,
    }
}
