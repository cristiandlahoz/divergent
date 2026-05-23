use once_cell::sync::Lazy;
use tree_sitter_highlight::HighlightConfiguration;

use super::queries::*;

// Elixir uses the bundled highlight queries from tree-sitter-elixir
const ELIXIR_HIGHLIGHTS: &str = tree_sitter_elixir::HIGHLIGHTS_QUERY;

pub const HIGHLIGHT_NAMES: &[&str] = &[
    "attribute",
    "comment",
    "constant",
    "constant.builtin",
    "constructor",
    "function",
    "function.builtin",
    "function.method",
    "function.macro",
    "keyword",
    "label",
    "module",
    "number",
    "operator",
    "property",
    "punctuation",
    "punctuation.bracket",
    "punctuation.delimiter",
    "string",
    "string.special",
    "tag",
    "type",
    "type.builtin",
    "variable",
    "variable.builtin",
    "variable.parameter",
    "variable.member",
];

pub struct LanguageConfig {
    pub config: HighlightConfiguration,
}

type ConfigCell = Lazy<Option<LanguageConfig>>;

pub struct LanguageEntry {
    pub extensions: &'static [&'static str],
    pub config: &'static ConfigCell,
}

fn load_config(
    language: tree_sitter::Language,
    name: &str,
    highlights: &str,
    extensions: &'static [&'static str],
) -> Option<LanguageConfig> {
    match HighlightConfiguration::new(language, name, highlights, "", "") {
        Ok(mut config) => {
            config.configure(HIGHLIGHT_NAMES);
            Some(LanguageConfig { config })
        }
        Err(_e) => {
            #[cfg(debug_assertions)]
            eprintln!(
                "[WARN] Failed to load {} highlight config for {:?}: {:?}",
                name, extensions, _e
            );
            #[cfg(not(debug_assertions))]
            let _ = extensions;
            None
        }
    }
}

static TYPESCRIPT_CONFIG: ConfigCell = Lazy::new(|| {
    load_config(
        tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
        "typescript",
        TS_HIGHLIGHTS,
        &["ts", "mts", "cts"],
    )
});
static TSX_CONFIG: ConfigCell = Lazy::new(|| {
    load_config(
        tree_sitter_typescript::LANGUAGE_TSX.into(),
        "tsx",
        TSX_HIGHLIGHTS,
        &["tsx"],
    )
});
static JAVASCRIPT_CONFIG: ConfigCell = Lazy::new(|| {
    load_config(
        tree_sitter_javascript::LANGUAGE.into(),
        "javascript",
        JS_HIGHLIGHTS,
        &["js", "jsx", "mjs", "cjs"],
    )
});
static RUST_CONFIG: ConfigCell = Lazy::new(|| {
    load_config(
        tree_sitter_rust::LANGUAGE.into(),
        "rust",
        RUST_HIGHLIGHTS,
        &["rs"],
    )
});
static JSON_CONFIG: ConfigCell = Lazy::new(|| {
    load_config(
        tree_sitter_json::LANGUAGE.into(),
        "json",
        JSON_HIGHLIGHTS,
        &["json"],
    )
});
static PYTHON_CONFIG: ConfigCell = Lazy::new(|| {
    load_config(
        tree_sitter_python::LANGUAGE.into(),
        "python",
        PYTHON_HIGHLIGHTS,
        &["py", "pyw"],
    )
});
static GO_CONFIG: ConfigCell = Lazy::new(|| {
    load_config(
        tree_sitter_go::LANGUAGE.into(),
        "go",
        GO_HIGHLIGHTS,
        &["go"],
    )
});
static CSS_CONFIG: ConfigCell = Lazy::new(|| {
    load_config(
        tree_sitter_css::LANGUAGE.into(),
        "css",
        CSS_HIGHLIGHTS,
        &["css"],
    )
});
static HTML_CONFIG: ConfigCell = Lazy::new(|| {
    load_config(
        tree_sitter_html::LANGUAGE.into(),
        "html",
        HTML_HIGHLIGHTS,
        &["html"],
    )
});
static TOML_CONFIG: ConfigCell = Lazy::new(|| {
    load_config(
        tree_sitter_toml_ng::LANGUAGE.into(),
        "toml",
        TOML_HIGHLIGHTS,
        &["toml"],
    )
});
static BASH_CONFIG: ConfigCell = Lazy::new(|| {
    load_config(
        tree_sitter_bash::LANGUAGE.into(),
        "bash",
        BASH_HIGHLIGHTS,
        &["sh", "bash", "zsh", "ksh", "bats"],
    )
});
static MARKDOWN_CONFIG: ConfigCell = Lazy::new(|| {
    load_config(
        tree_sitter_md::LANGUAGE.into(),
        "markdown",
        MD_HIGHLIGHTS,
        &["md", "mdx", "markdown"],
    )
});
static CSHARP_CONFIG: ConfigCell = Lazy::new(|| {
    load_config(
        tree_sitter_c_sharp::LANGUAGE.into(),
        "c_sharp",
        CSHARP_HIGHLIGHTS,
        &["cs", "csx"],
    )
});
static JAVA_CONFIG: ConfigCell = Lazy::new(|| {
    load_config(
        tree_sitter_java::LANGUAGE.into(),
        "java",
        JAVA_HIGHLIGHTS,
        &["java"],
    )
});
static RUBY_CONFIG: ConfigCell = Lazy::new(|| {
    load_config(
        tree_sitter_ruby::LANGUAGE.into(),
        "ruby",
        RUBY_HIGHLIGHTS,
        &["rb", "rake", "gemspec"],
    )
});
static ELIXIR_CONFIG: ConfigCell = Lazy::new(|| {
    load_config(
        tree_sitter_elixir::LANGUAGE.into(),
        "elixir",
        ELIXIR_HIGHLIGHTS,
        &["ex", "exs"],
    )
});
static ZIG_CONFIG: ConfigCell = Lazy::new(|| {
    load_config(
        tree_sitter_zig::LANGUAGE.into(),
        "zig",
        ZIG_HIGHLIGHTS,
        &["zig"],
    )
});
static YAML_CONFIG: ConfigCell = Lazy::new(|| {
    load_config(
        tree_sitter_yaml::LANGUAGE.into(),
        "yaml",
        tree_sitter_yaml::HIGHLIGHTS_QUERY,
        &["yaml", "yml"],
    )
});

pub static LANGUAGES: &[LanguageEntry] = &[
    LanguageEntry {
        extensions: &["ts", "mts", "cts"],
        config: &TYPESCRIPT_CONFIG,
    },
    LanguageEntry {
        extensions: &["tsx"],
        config: &TSX_CONFIG,
    },
    LanguageEntry {
        extensions: &["js", "jsx", "mjs", "cjs"],
        config: &JAVASCRIPT_CONFIG,
    },
    LanguageEntry {
        extensions: &["rs"],
        config: &RUST_CONFIG,
    },
    LanguageEntry {
        extensions: &["json"],
        config: &JSON_CONFIG,
    },
    LanguageEntry {
        extensions: &["py", "pyw"],
        config: &PYTHON_CONFIG,
    },
    LanguageEntry {
        extensions: &["go"],
        config: &GO_CONFIG,
    },
    LanguageEntry {
        extensions: &["css"],
        config: &CSS_CONFIG,
    },
    LanguageEntry {
        extensions: &["html"],
        config: &HTML_CONFIG,
    },
    LanguageEntry {
        extensions: &["toml"],
        config: &TOML_CONFIG,
    },
    LanguageEntry {
        extensions: &["sh", "bash", "zsh", "ksh", "bats"],
        config: &BASH_CONFIG,
    },
    LanguageEntry {
        extensions: &["md", "mdx", "markdown"],
        config: &MARKDOWN_CONFIG,
    },
    LanguageEntry {
        extensions: &["cs", "csx"],
        config: &CSHARP_CONFIG,
    },
    LanguageEntry {
        extensions: &["java"],
        config: &JAVA_CONFIG,
    },
    LanguageEntry {
        extensions: &["rb", "rake", "gemspec"],
        config: &RUBY_CONFIG,
    },
    LanguageEntry {
        extensions: &["ex", "exs"],
        config: &ELIXIR_CONFIG,
    },
    LanguageEntry {
        extensions: &["zig"],
        config: &ZIG_CONFIG,
    },
    LanguageEntry {
        extensions: &["yaml", "yml"],
        config: &YAML_CONFIG,
    },
];

pub fn get_config_for_extension(ext: &str) -> Option<&'static LanguageConfig> {
    let ext = ext.to_ascii_lowercase();
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
