use once_cell::sync::Lazy;
use tree_sitter_highlight::HighlightConfiguration;

use super::queries::*;

const ELIXIR_HIGHLIGHTS: &str = tree_sitter_elixir::HIGHLIGHTS_QUERY;
const XML_HIGHLIGHTS: &str = tree_sitter_xml::XML_HIGHLIGHT_QUERY;

pub const HIGHLIGHT_NAMES: &[&str] = &[
    "attribute",
    "boolean",
    "comment",
    "constant",
    "constant.builtin",
    "constructor",
    "embedded",
    "error",
    "function",
    "function.builtin",
    "function.method",
    "function.macro",
    "keyword",
    "label",
    "markup",
    "markup.heading",
    "markup.link",
    "markup.raw",
    "module",
    "number",
    "operator",
    "property",
    "punctuation",
    "punctuation.bracket",
    "punctuation.delimiter",
    "string",
    "string.special",
    "string.special.symbol",
    "tag",
    "type",
    "type.builtin",
    "variable",
    "variable.builtin",
    "variable.member",
    "variable.parameter",
];

pub struct LanguageConfig {
    #[cfg(test)]
    pub name: &'static str,
    pub config: HighlightConfiguration,
}

type ConfigCell = Lazy<Option<LanguageConfig>>;

pub struct LanguageEntry {
    pub extensions: &'static [&'static str],
    pub exact_filenames: &'static [&'static str],
    pub config: &'static ConfigCell,
}

fn load_config(
    language: tree_sitter::Language,
    name: &'static str,
    highlights: &str,
) -> Option<LanguageConfig> {
    match HighlightConfiguration::new(language, name, highlights, "", "") {
        Ok(mut config) => {
            config.configure(HIGHLIGHT_NAMES);
            Some(LanguageConfig {
                #[cfg(test)]
                name,
                config,
            })
        }
        Err(_e) => {
            #[cfg(debug_assertions)]
            eprintln!("[WARN] Failed to load {name} highlight config: {_e:?}");
            None
        }
    }
}

static TYPESCRIPT_CONFIG: ConfigCell = Lazy::new(|| {
    load_config(
        tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
        "typescript",
        TS_HIGHLIGHTS,
    )
});
static TSX_CONFIG: ConfigCell = Lazy::new(|| {
    load_config(
        tree_sitter_typescript::LANGUAGE_TSX.into(),
        "tsx",
        TSX_HIGHLIGHTS,
    )
});
static JAVASCRIPT_CONFIG: ConfigCell = Lazy::new(|| {
    load_config(
        tree_sitter_javascript::LANGUAGE.into(),
        "javascript",
        JS_HIGHLIGHTS,
    )
});
static RUST_CONFIG: ConfigCell =
    Lazy::new(|| load_config(tree_sitter_rust::LANGUAGE.into(), "rust", RUST_HIGHLIGHTS));
static JSON_CONFIG: ConfigCell =
    Lazy::new(|| load_config(tree_sitter_json::LANGUAGE.into(), "json", JSON_HIGHLIGHTS));
static PYTHON_CONFIG: ConfigCell = Lazy::new(|| {
    load_config(
        tree_sitter_python::LANGUAGE.into(),
        "python",
        PYTHON_HIGHLIGHTS,
    )
});
static GO_CONFIG: ConfigCell =
    Lazy::new(|| load_config(tree_sitter_go::LANGUAGE.into(), "go", GO_HIGHLIGHTS));
static CSS_CONFIG: ConfigCell =
    Lazy::new(|| load_config(tree_sitter_css::LANGUAGE.into(), "css", CSS_HIGHLIGHTS));
static HTML_CONFIG: ConfigCell =
    Lazy::new(|| load_config(tree_sitter_html::LANGUAGE.into(), "html", HTML_HIGHLIGHTS));
static TOML_CONFIG: ConfigCell = Lazy::new(|| {
    load_config(
        tree_sitter_toml_ng::LANGUAGE.into(),
        "toml",
        TOML_HIGHLIGHTS,
    )
});
static BASH_CONFIG: ConfigCell =
    Lazy::new(|| load_config(tree_sitter_bash::LANGUAGE.into(), "bash", BASH_HIGHLIGHTS));
static MARKDOWN_CONFIG: ConfigCell =
    Lazy::new(|| load_config(tree_sitter_md::LANGUAGE.into(), "markdown", MD_HIGHLIGHTS));
static CSHARP_CONFIG: ConfigCell = Lazy::new(|| {
    load_config(
        tree_sitter_c_sharp::LANGUAGE.into(),
        "c_sharp",
        CSHARP_HIGHLIGHTS,
    )
});
static JAVA_CONFIG: ConfigCell =
    Lazy::new(|| load_config(tree_sitter_java::LANGUAGE.into(), "java", JAVA_HIGHLIGHTS));
static RUBY_CONFIG: ConfigCell =
    Lazy::new(|| load_config(tree_sitter_ruby::LANGUAGE.into(), "ruby", RUBY_HIGHLIGHTS));
static ELIXIR_CONFIG: ConfigCell = Lazy::new(|| {
    load_config(
        tree_sitter_elixir::LANGUAGE.into(),
        "elixir",
        ELIXIR_HIGHLIGHTS,
    )
});
static ZIG_CONFIG: ConfigCell =
    Lazy::new(|| load_config(tree_sitter_zig::LANGUAGE.into(), "zig", ZIG_HIGHLIGHTS));
static YAML_CONFIG: ConfigCell = Lazy::new(|| {
    load_config(
        tree_sitter_yaml::LANGUAGE.into(),
        "yaml",
        tree_sitter_yaml::HIGHLIGHTS_QUERY,
    )
});
static XML_CONFIG: ConfigCell =
    Lazy::new(|| load_config(tree_sitter_xml::LANGUAGE_XML.into(), "xml", XML_HIGHLIGHTS));
static POSTGRES_CONFIG: ConfigCell = Lazy::new(|| {
    load_config(
        tree_sitter_postgres::LANGUAGE.into(),
        "postgres",
        POSTGRES_HIGHLIGHTS,
    )
});

pub static LANGUAGES: &[LanguageEntry] = &[
    LanguageEntry {
        extensions: &["ts", "mts", "cts"],
        exact_filenames: &[],
        config: &TYPESCRIPT_CONFIG,
    },
    LanguageEntry {
        extensions: &["tsx"],
        exact_filenames: &[],
        config: &TSX_CONFIG,
    },
    LanguageEntry {
        extensions: &["js", "jsx", "mjs", "cjs"],
        exact_filenames: &[],
        config: &JAVASCRIPT_CONFIG,
    },
    LanguageEntry {
        extensions: &["rs"],
        exact_filenames: &[],
        config: &RUST_CONFIG,
    },
    LanguageEntry {
        extensions: &["json"],
        exact_filenames: &[],
        config: &JSON_CONFIG,
    },
    LanguageEntry {
        extensions: &["py", "pyw"],
        exact_filenames: &[],
        config: &PYTHON_CONFIG,
    },
    LanguageEntry {
        extensions: &["go"],
        exact_filenames: &[],
        config: &GO_CONFIG,
    },
    LanguageEntry {
        extensions: &["css"],
        exact_filenames: &[],
        config: &CSS_CONFIG,
    },
    LanguageEntry {
        extensions: &["html", "htm"],
        exact_filenames: &[],
        config: &HTML_CONFIG,
    },
    LanguageEntry {
        extensions: &["toml"],
        exact_filenames: &[],
        config: &TOML_CONFIG,
    },
    LanguageEntry {
        extensions: &["sh", "bash", "zsh", "ksh", "bats"],
        exact_filenames: &[".bashrc", ".bash_profile", ".zshrc", ".profile"],
        config: &BASH_CONFIG,
    },
    LanguageEntry {
        extensions: &["md", "mdx", "markdown"],
        exact_filenames: &[],
        config: &MARKDOWN_CONFIG,
    },
    LanguageEntry {
        extensions: &["cs", "csx"],
        exact_filenames: &[],
        config: &CSHARP_CONFIG,
    },
    LanguageEntry {
        extensions: &["java"],
        exact_filenames: &[],
        config: &JAVA_CONFIG,
    },
    LanguageEntry {
        extensions: &["rb", "rake", "gemspec"],
        exact_filenames: &["Gemfile", "Rakefile"],
        config: &RUBY_CONFIG,
    },
    LanguageEntry {
        extensions: &["ex", "exs"],
        exact_filenames: &[],
        config: &ELIXIR_CONFIG,
    },
    LanguageEntry {
        extensions: &["zig"],
        exact_filenames: &[],
        config: &ZIG_CONFIG,
    },
    LanguageEntry {
        extensions: &["yaml", "yml"],
        exact_filenames: &[],
        config: &YAML_CONFIG,
    },
    LanguageEntry {
        extensions: &["xml", "xsd", "xsl", "xslt", "svg"],
        exact_filenames: &[],
        config: &XML_CONFIG,
    },
    LanguageEntry {
        extensions: &["sql", "psql"],
        exact_filenames: &[],
        config: &POSTGRES_CONFIG,
    },
];

pub fn get_config_for_file(filename: &str) -> Option<&'static LanguageConfig> {
    let path = std::path::Path::new(filename);
    if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
        if let Some(entry) = LANGUAGES
            .iter()
            .find(|entry| entry.exact_filenames.contains(&file_name))
        {
            return entry.config.as_ref();
        }
    }

    let ext = path
        .extension()
        .and_then(|e| e.to_str())?
        .to_ascii_lowercase();
    LANGUAGES
        .iter()
        .find(|entry| entry.extensions.contains(&ext.as_str()))
        .and_then(|entry| entry.config.as_ref())
}

#[cfg(test)]
pub fn supported_extensions() -> Vec<&'static str> {
    LANGUAGES
        .iter()
        .flat_map(|entry| entry.extensions.iter().copied())
        .collect()
}
