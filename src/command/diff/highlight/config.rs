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
    "variable.parameter",
    "variable.member",
    "embedded",
    "error",
];

pub struct LanguageConfig {
    #[cfg(test)]
    pub name: &'static str,
    pub config: HighlightConfiguration,
    extensions: &'static [&'static str],
    exact_filenames: &'static [&'static str],
}

impl LanguageConfig {
    pub fn matches_extension(&self, extension: &str) -> bool {
        self.extensions.contains(&extension)
    }

    pub fn matches_filename(&self, filename: &str) -> bool {
        self.exact_filenames.contains(&filename)
    }
}

struct LanguageSpec {
    language: tree_sitter::Language,
    name: &'static str,
    highlights: &'static str,
    extensions: &'static [&'static str],
    exact_filenames: &'static [&'static str],
}

fn load_config(spec: LanguageSpec, configs: &mut Vec<LanguageConfig>) {
    match HighlightConfiguration::new(spec.language, spec.name, spec.highlights, "", "") {
        Ok(mut config) => {
            config.configure(HIGHLIGHT_NAMES);
            configs.push(LanguageConfig {
                #[cfg(test)]
                name: spec.name,
                config,
                extensions: spec.extensions,
                exact_filenames: spec.exact_filenames,
            });
        }
        Err(_e) => {
            #[cfg(debug_assertions)]
            eprintln!(
                "[WARN] Failed to load {} highlight config: {:?}",
                spec.name, _e
            );
        }
    }
}

pub static CONFIGS: Lazy<Vec<LanguageConfig>> = Lazy::new(|| {
    let mut configs = Vec::new();

    load_config(
        LanguageSpec {
            language: tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            name: "typescript",
            highlights: TS_HIGHLIGHTS,
            extensions: &["ts", "mts", "cts"],
            exact_filenames: &[],
        },
        &mut configs,
    );

    load_config(
        LanguageSpec {
            language: tree_sitter_typescript::LANGUAGE_TSX.into(),
            name: "tsx",
            highlights: TSX_HIGHLIGHTS,
            extensions: &["tsx"],
            exact_filenames: &[],
        },
        &mut configs,
    );

    load_config(
        LanguageSpec {
            language: tree_sitter_javascript::LANGUAGE.into(),
            name: "javascript",
            highlights: JS_HIGHLIGHTS,
            extensions: &["js", "jsx", "mjs", "cjs"],
            exact_filenames: &[],
        },
        &mut configs,
    );

    load_config(
        LanguageSpec {
            language: tree_sitter_rust::LANGUAGE.into(),
            name: "rust",
            highlights: RUST_HIGHLIGHTS,
            extensions: &["rs"],
            exact_filenames: &[],
        },
        &mut configs,
    );

    load_config(
        LanguageSpec {
            language: tree_sitter_json::LANGUAGE.into(),
            name: "json",
            highlights: JSON_HIGHLIGHTS,
            extensions: &["json"],
            exact_filenames: &[],
        },
        &mut configs,
    );

    load_config(
        LanguageSpec {
            language: tree_sitter_python::LANGUAGE.into(),
            name: "python",
            highlights: PYTHON_HIGHLIGHTS,
            extensions: &["py", "pyw"],
            exact_filenames: &[],
        },
        &mut configs,
    );

    load_config(
        LanguageSpec {
            language: tree_sitter_go::LANGUAGE.into(),
            name: "go",
            highlights: GO_HIGHLIGHTS,
            extensions: &["go"],
            exact_filenames: &[],
        },
        &mut configs,
    );

    load_config(
        LanguageSpec {
            language: tree_sitter_css::LANGUAGE.into(),
            name: "css",
            highlights: CSS_HIGHLIGHTS,
            extensions: &["css"],
            exact_filenames: &[],
        },
        &mut configs,
    );

    load_config(
        LanguageSpec {
            language: tree_sitter_html::LANGUAGE.into(),
            name: "html",
            highlights: HTML_HIGHLIGHTS,
            extensions: &["html", "htm"],
            exact_filenames: &[],
        },
        &mut configs,
    );

    load_config(
        LanguageSpec {
            language: tree_sitter_toml_ng::LANGUAGE.into(),
            name: "toml",
            highlights: TOML_HIGHLIGHTS,
            extensions: &["toml"],
            exact_filenames: &[],
        },
        &mut configs,
    );

    load_config(
        LanguageSpec {
            language: tree_sitter_bash::LANGUAGE.into(),
            name: "bash",
            highlights: BASH_HIGHLIGHTS,
            extensions: &["sh", "bash", "zsh", "ksh", "bats"],
            exact_filenames: &[".bashrc", ".bash_profile", ".zshrc", ".profile"],
        },
        &mut configs,
    );

    load_config(
        LanguageSpec {
            language: tree_sitter_md::LANGUAGE.into(),
            name: "markdown",
            highlights: MD_HIGHLIGHTS,
            extensions: &["md", "mdx", "markdown"],
            exact_filenames: &[],
        },
        &mut configs,
    );

    load_config(
        LanguageSpec {
            language: tree_sitter_c_sharp::LANGUAGE.into(),
            name: "c_sharp",
            highlights: CSHARP_HIGHLIGHTS,
            extensions: &["cs", "csx"],
            exact_filenames: &[],
        },
        &mut configs,
    );

    load_config(
        LanguageSpec {
            language: tree_sitter_java::LANGUAGE.into(),
            name: "java",
            highlights: JAVA_HIGHLIGHTS,
            extensions: &["java"],
            exact_filenames: &[],
        },
        &mut configs,
    );

    load_config(
        LanguageSpec {
            language: tree_sitter_ruby::LANGUAGE.into(),
            name: "ruby",
            highlights: RUBY_HIGHLIGHTS,
            extensions: &["rb", "rake", "gemspec"],
            exact_filenames: &["Gemfile", "Rakefile"],
        },
        &mut configs,
    );

    load_config(
        LanguageSpec {
            language: tree_sitter_elixir::LANGUAGE.into(),
            name: "elixir",
            highlights: ELIXIR_HIGHLIGHTS,
            extensions: &["ex", "exs"],
            exact_filenames: &[],
        },
        &mut configs,
    );

    load_config(
        LanguageSpec {
            language: tree_sitter_zig::LANGUAGE.into(),
            name: "zig",
            highlights: ZIG_HIGHLIGHTS,
            extensions: &["zig"],
            exact_filenames: &[],
        },
        &mut configs,
    );

    load_config(
        LanguageSpec {
            language: tree_sitter_xml::LANGUAGE_XML.into(),
            name: "xml",
            highlights: XML_HIGHLIGHTS,
            extensions: &["xml", "xsd", "xsl", "xslt", "svg", "rss"],
            exact_filenames: &[],
        },
        &mut configs,
    );

    configs
});
