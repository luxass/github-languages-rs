use quote::{format_ident, quote};
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=languages.yml");
    let dest_path = Path::new("./src/generated.rs");
    let is_docs_rs = env::var("DOCS_RS").is_ok();

    let contents = std::fs::read_to_string("./languages.yml")?;
    let languages: serde_yaml::Value = serde_yaml::from_str(&contents)?;

    let mut language_structs = Vec::new();
    let mut language_impls = Vec::new();
    let mut language_info_calls = Vec::new();

    if let serde_yaml::Value::Mapping(map) = languages {
        for (key, value) in map {
            if let (serde_yaml::Value::String(name), serde_yaml::Value::Mapping(lang_map)) =
                (key, value)
            {
                println!("{:?}", lang_map);
                let struct_name = sanitize_name(&name);
                let struct_ident = format_ident!("{}", struct_name);

                let extensions: Vec<String> = lang_map
                    .get("extensions")
                    .and_then(|v| v.as_sequence())
                    .map(|seq| {
                        seq.iter()
                            .filter_map(|v| v.as_str().map(|s| s.to_string()))
                            .collect()
                    })
                    .unwrap_or_default();

                language_structs.push(quote! {
                    pub struct #struct_ident;
                });

                let _type = lang_map
                    .get("type")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();

                let color = lang_map.get("color").and_then(|v| v.as_str()).unwrap_or("");

                let tm_scope = lang_map
                    .get("tm_scope")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();

                let ace_mode = lang_map
                    .get("ace_mode")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();

                let language_id = lang_map
                    .get("language_id")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0);

                let aliases: Vec<String> = lang_map
                    .get("aliases")
                    .and_then(|v| v.as_sequence())
                    .map(|seq| {
                        seq.iter()
                            .filter_map(|v| v.as_str().map(|s| s.to_string()))
                            .collect()
                    })
                    .unwrap_or_default();

                let codemirror_mode = lang_map
                    .get("codemirror_mode")
                    .and_then(|v| v.as_str())
                    .map(|s| quote!(Some(#s)))
                    .unwrap_or(quote!(None));

                let codemirror_mime_type = lang_map
                    .get("codemirror_mime_type")
                    .and_then(|v| v.as_str())
                    .map(|s| quote!(Some(#s)))
                    .unwrap_or(quote!(None));

                let wrap = lang_map
                    .get("wrap")
                    .and_then(|v| v.as_bool())
                    .map(|b| quote!(Some(#b)))
                    .unwrap_or(quote!(None));

                let filenames: Vec<String> = lang_map
                    .get("filenames")
                    .and_then(|v| v.as_sequence())
                    .map(|seq| {
                        seq.iter()
                            .filter_map(|v| v.as_str().map(|s| s.to_string()))
                            .collect()
                    })
                    .unwrap_or_default();

                let group = lang_map
                    .get("group")
                    .and_then(|v| v.as_str())
                    .map(|s| quote!(Some(#s)))
                    .unwrap_or(quote!(None));

                let searchable = lang_map
                    .get("searchable")
                    .and_then(|v| v.as_bool())
                    .map(|b| quote!(Some(#b)))
                    .unwrap_or(quote!(None));

                let fs_name = lang_map
                    .get("fs_name")
                    .and_then(|v| v.as_str())
                    .map(|s| quote!(Some(#s)))
                    .unwrap_or(quote!(None));

                let interpreters: Vec<String> = lang_map
                    .get("interpreters")
                    .and_then(|v| v.as_sequence())
                    .map(|seq| {
                        seq.iter()
                            .filter_map(|v| v.as_str().map(|s| s.to_string()))
                            .collect()
                    })
                    .unwrap_or_default();

                language_impls.push(quote! {
                    impl #struct_ident {
                        pub fn info() -> Language {
                            Language {
                                name: #name,
                                r#type: #_type,
                                color: #color,
                                extensions: &[#(#extensions),*],
                                tm_scope: #tm_scope,
                                ace_mode: #ace_mode,
                                language_id: #language_id,
                                aliases: &[#(#aliases),*],
                                codemirror_mode: #codemirror_mode,
                                codemirror_mime_type: #codemirror_mime_type,
                                wrap: #wrap,
                                filenames: &[#(#filenames),*],
                                group: #group,
                                interpreters: &[#(#interpreters),*],
                                fs_name: #fs_name,
                                searchable: #searchable,
                            }
                        }
                    }
                });

                language_info_calls.push(quote! {
                    #struct_ident::info()
                });
            }
        }
    }

    let output = quote! {
        use std::collections::HashMap;
        use once_cell::sync::Lazy;

        #[derive(Debug, Clone, PartialEq)]
        pub struct Language {
            pub name: &'static str,
            pub r#type: &'static str,
            pub color: &'static str,
            pub extensions: &'static [&'static str],
            pub aliases: &'static [&'static str],
            pub tm_scope: &'static str,
            pub ace_mode: &'static str,
            pub language_id: u64,
            pub codemirror_mode: Option<&'static str>,
            pub codemirror_mime_type: Option<&'static str>,
            pub wrap: Option<bool>,
            pub filenames: &'static [&'static str],
            pub group: Option<&'static str>,
            pub interpreters: &'static [&'static str],
            pub fs_name: Option<&'static str>,
            pub searchable: Option<bool>,
        }

        #(#language_structs)*

        #(#language_impls)*

        pub struct Languages {
            languages: Vec<Language>,
            by_name: HashMap<&'static str, usize>,
            by_extension: HashMap<&'static str, Vec<usize>>,
        }

        impl Languages {
            pub fn new() -> Self {
                let languages = vec![
                    #(#language_info_calls),*
                ];

                let mut by_name = HashMap::new();
                let mut by_extension = HashMap::new();

                for (index, lang) in languages.iter().enumerate() {
                    by_name.insert(lang.name, index);
                    for ext in lang.extensions {
                        by_extension.entry(*ext).or_insert_with(Vec::new).push(index);
                    }
                }

                Self { languages, by_name, by_extension }
            }

            pub fn get_by_name(&self, name: &str) -> Option<&Language> {
                self.by_name.get(name).map(|&index| &self.languages[index])
            }

            pub fn get_by_extension(&self, ext: &str) -> Vec<&Language> {
                self.by_extension.get(ext)
                    .map(|indices| indices.iter().map(|&index| &self.languages[index]).collect())
                    .unwrap_or_default()
            }

            pub fn all_languages(&self) -> &[Language] {
                &self.languages
            }
        }

        impl Default for Languages {
            fn default() -> Self {
                Self::new()
            }
        }

        pub fn get_languages() -> Languages {
            Languages::new()
        }

        pub static LANGUAGES: Lazy<Languages> = Lazy::new(Languages::new);
    };

    if is_docs_rs {
        println!("cargo:warning=Generated code:\n{}", output);
    } else {
        let mut file = File::create(dest_path)?;
        let syntax_tree = syn::parse2(output)?;
        let formatted = prettyplease::unparse(&syntax_tree);
        file.write_all(formatted.as_bytes())?;
    }

    Ok(())
}

fn sanitize_name(name: &str) -> String {
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

    if struct_name == "Self" {
        struct_name = "_Self".to_string();
    }

    if struct_name.starts_with(char::is_numeric) {
        struct_name = format!("_{}", struct_name);
    }

    struct_name
}
