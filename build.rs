use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get("https://raw.githubusercontent.com/github-linguist/linguist/refs/heads/main/lib/linguist/languages.yml")?;

    if !response.status().is_success() {
        return Err("failed to fetch languages.yml".into());
    }

    let dest_path = Path::new("./src/generated.rs");

    let contents = response.text()?;

    let languages: serde_yaml::Value = serde_yaml::from_str(&contents)?;

    let mut output = String::new();
    output.push_str("use std::collections::HashMap;\n");
    output.push_str("use once_cell::sync::Lazy;\n\n");

    output.push_str("#[derive(Debug, Clone, PartialEq)]\n");
    output.push_str("pub struct Language {\n");
    output.push_str("    pub name: &'static str,\n");
    output.push_str("    pub type_: &'static str,\n");
    output.push_str("    pub color: &'static str,\n");
    output.push_str("    pub extensions: &'static [&'static str],\n");
    output.push_str("    pub tm_scope: &'static str,\n");
    output.push_str("    pub ace_mode: &'static str,\n");
    output.push_str("    pub language_id: u64,\n");
    output.push_str("}\n\n");

    let mut language_structs = Vec::new();

    if let serde_yaml::Value::Mapping(map) = languages {
        for (key, value) in map {
            if let (serde_yaml::Value::String(name), serde_yaml::Value::Mapping(lang_map)) =
                (key, value)
            {
                let struct_name = sanitize_name(&name);

                let lang_struct = format!(
                    "pub struct {};\n\n\
                     impl {} {{\n\
                         pub fn info() -> Language {{\n\
                             Language {{\n\
                                 name: \"{}\",\n\
                                 type_: \"{}\",\n\
                                 color: \"{}\",\n\
                                 extensions: &[{}],\n\
                                 tm_scope: \"{}\",\n\
                                 ace_mode: \"{}\",\n\
                                 language_id: {},\n\
                             }}\n\
                         }}\n\
                     }}\n\n",
                    struct_name,
                    struct_name,
                    name,
                    lang_map.get("type").and_then(|v| v.as_str()).unwrap_or(""),
                    lang_map.get("color").and_then(|v| v.as_str()).unwrap_or(""),
                    lang_map
                        .get("extensions")
                        .and_then(|v| v.as_sequence())
                        .map(|seq| seq
                            .iter()
                            .filter_map(|v| v.as_str())
                            .map(|s| format!("\"{}\"", s))
                            .collect::<Vec<_>>()
                            .join(", "))
                        .unwrap_or_else(|| "".to_string()),
                    lang_map
                        .get("tm_scope")
                        .and_then(|v| v.as_str())
                        .unwrap_or(""),
                    lang_map
                        .get("ace_mode")
                        .and_then(|v| v.as_str())
                        .unwrap_or(""),
                    lang_map
                        .get("language_id")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(0)
                );

                language_structs.push((struct_name, lang_struct));
            }
        }
    }

    // Write individual language structs
    for (_, lang_struct) in &language_structs {
        output.push_str(lang_struct);
    }

    // Generate Languages struct and implementation
    output.push_str("pub struct Languages {\n");
    output.push_str("    languages: Vec<Language>,\n");
    output.push_str("    by_name: HashMap<&'static str, usize>,\n");
    output.push_str("    by_extension: HashMap<&'static str, usize>,\n");
    output.push_str("}\n\n");

    output.push_str("impl Languages {\n");
    output.push_str("    pub fn new() -> Self {\n");
    output.push_str("        let languages = vec![\n");

    for (struct_name, _) in &language_structs {
        output.push_str(&format!("            {}::info(),\n", struct_name));
    }

    output.push_str("        ];\n\n");
    output.push_str("        let mut by_name = HashMap::new();\n");
    output.push_str("        let mut by_extension = HashMap::new();\n\n");
    output.push_str("        for (index, lang) in languages.iter().enumerate() {\n");
    output.push_str("            by_name.insert(lang.name, index);\n");
    output.push_str("            for ext in lang.extensions {\n");
    output.push_str("                by_extension.insert(*ext, index);\n");
    output.push_str("            }\n");
    output.push_str("        }\n\n");
    output.push_str("        Self { languages, by_name, by_extension }\n");
    output.push_str("    }\n\n");

    output.push_str("    pub fn get_by_name(&self, name: &str) -> Option<&Language> {\n");
    output.push_str("        self.by_name.get(name).map(|&index| &self.languages[index])\n");
    output.push_str("    }\n\n");

    output.push_str("    pub fn get_by_extension(&self, ext: &str) -> Option<&Language> {\n");
    output.push_str("        self.by_extension.get(ext).map(|&index| &self.languages[index])\n");
    output.push_str("    }\n\n");

    output.push_str("    pub fn all_languages(&self) -> &[Language] {\n");
    output.push_str("        &self.languages\n");
    output.push_str("    }\n");
    output.push_str("}\n\n");

    output.push_str("pub fn get_languages() -> Languages {\n");
    output.push_str("    Languages::new()\n");
    output.push_str("}\n\n");

    output.push_str("impl Default for Languages {\n");
    output.push_str("    fn default() -> Self {\n");
    output.push_str("        Self::new()\n");
    output.push_str("    }\n");
    output.push_str("}\n\n");

    output.push_str("pub static LANGUAGES: Lazy<Languages> = Lazy::new(Languages::new);\n");

    let mut file = File::create(dest_path)?;
    let syntax_tree = syn::parse_file(&output)?;
    let formatted = prettyplease::unparse(&syntax_tree);
    file.write_all(formatted.as_bytes())?;

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
