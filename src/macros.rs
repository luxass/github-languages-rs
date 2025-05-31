#[derive(Debug, Clone, PartialEq)]
pub struct LanguageInfo {
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

#[macro_export]
macro_rules! define_languages {
    (
        $(
            $struct_name:ident => {
                name: $name:literal,
                r#type: $lang_type:literal,
                color: $color:literal,
                extensions: [ $( $ext:literal ),* $( , )? ],
                aliases: [ $( $alias:literal ),* $( , )? ],
                tm_scope: $tm_scope:literal,
                ace_mode: $ace_mode:literal,
                language_id: $id:literal,
                $( codemirror_mode: $cm_mode:literal, )?
                $( codemirror_mime_type: $cm_mime:literal, )?
                $( wrap: $wrap:literal, )?
                filenames: [ $( $filename:literal ),* $( , )? ],
                $( group: $group:literal, )?
                interpreters: [ $( $interp:literal ),* $( , )? ],
                $( fs_name: $fs_name:literal, )?
                $( searchable: $searchable:literal, )?
            }
        ),*
        $( , )?
    ) => {
        use $crate::macros::LanguageInfo;
        use once_cell::sync::Lazy;

        // Generate simple language structs (no modules!)
        $(
            pub struct $struct_name;

            impl $struct_name {
                pub fn info() -> LanguageInfo {
                    LanguageInfo {
                        name: $name,
                        r#type: $lang_type,
                        color: $color,
                        extensions: &[ $( $ext ),* ],
                        aliases: &[ $( $alias ),* ],
                        tm_scope: $tm_scope,
                        ace_mode: $ace_mode,
                        language_id: $id,
                        codemirror_mode: None $(.or(Some($cm_mode)))?,
                        codemirror_mime_type: None $(.or(Some($cm_mime)))?,
                        wrap: None $(.or(Some($wrap)))?,
                        filenames: &[ $( $filename ),* ],
                        group: None $(.or(Some($group)))?,
                        interpreters: &[ $( $interp ),* ],
                        fs_name: None $(.or(Some($fs_name)))?,
                        searchable: None $(.or(Some($searchable)))?,
                    }
                }
            }
        )*

        // Single Languages struct
        pub struct Languages;

        impl Languages {
            pub fn get_by_name(&self, name: &str) -> Option<LanguageInfo> {
                BY_NAME.get(name).map(|f| f())
            }

            pub fn get_by_id(&self, id: u64) -> Option<LanguageInfo> {
                BY_ID.get(&id).map(|f| f())
            }

            pub fn get_by_extension(&self, ext: &str) -> Vec<LanguageInfo> {
                BY_EXTENSION
                    .get(ext)
                    .map(|funcs| funcs.iter().map(|f| f()).collect())
                    .unwrap_or_default()
            }

            pub fn get_by_alias(&self, alias: &str) -> Vec<LanguageInfo> {
                ALL_LANGUAGES.iter().filter(|lang| lang.aliases.contains(&alias)).cloned().collect()
            }

            pub fn get_by_filename(&self, filename: &str) -> Vec<LanguageInfo> {
                ALL_LANGUAGES.iter().filter(|lang| lang.filenames.contains(&filename)).cloned().collect()
            }

            pub fn get_by_interpreter(&self, interpreter: &str) -> Vec<LanguageInfo> {
                ALL_LANGUAGES.iter().filter(|lang| lang.interpreters.contains(&interpreter)).cloned().collect()
            }

            pub fn get_by_type(&self, r#type: &str) -> Vec<LanguageInfo> {
                ALL_LANGUAGES.iter().filter(|lang| lang.r#type == r#type).cloned().collect()
            }

            pub fn all_languages(&self) -> &[LanguageInfo] {
                &ALL_LANGUAGES
            }
        }

        // Use Lazy initialization for the languages array
        static ALL_LANGUAGES: Lazy<Vec<LanguageInfo>> = Lazy::new(|| vec![
            $( $struct_name::info() ),*
        ]);

        static BY_NAME: phf::Map<&'static str, fn() -> LanguageInfo> = phf_map! {
            $( $name => $struct_name::info ),*
        };

        static BY_ID: phf::Map<u64, fn() -> LanguageInfo> = phf_map! {
            $( $id => $struct_name::info ),*
        };

        pub fn get_languages() -> Languages {
            Languages
        }

        pub static LANGUAGES: Lazy<Languages> = Lazy::new(|| Languages);
    };
}
