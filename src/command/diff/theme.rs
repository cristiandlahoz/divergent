//! Terminal palettes for the diff viewer.
//!
//! Divergent keeps theme selection intentionally small: Tokyo Night is the
//! default, automatic terminal detection remains available, and the other named
//! palette exists for users who want a stable look across terminals.

use clap::ValueEnum;
use once_cell::sync::OnceCell;
use ratatui::prelude::Color;
use std::str::FromStr;

static THEME: OnceCell<Theme> = OnceCell::new();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    /// Palette tuned for dark terminal backgrounds.
    Dark,
    /// Palette tuned for light terminal backgrounds.
    Light,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum ThemeChoice {
    /// Follow the terminal's detected light/dark mode.
    Auto,
    /// Alias for auto; useful for users who think in terms of defaults.
    Default,
    /// Tokyo Night-inspired dark palette.
    Tokyonight,
    /// Very dark, low-glare palette.
    Midnight,
}

impl FromStr for ThemeChoice {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "auto" => Ok(Self::Auto),
            "default" => Ok(Self::Default),
            "tokyonight" | "tokyo-night" => Ok(Self::Tokyonight),
            "midnight" | "mid-night" => Ok(Self::Midnight),
            _ => Err(format!(
                "invalid theme '{value}'. expected one of: auto, default, tokyonight, midnight"
            )),
        }
    }
}

impl ThemeMode {
    /// Detects the terminal luminance bucket instead of honoring a named preset.
    ///
    /// The fallback is dark because most terminal themes are dark and because a
    /// dark palette on a light background is usually more readable than the
    /// reverse failure mode.
    pub fn detect() -> Self {
        match terminal_light::luma() {
            Ok(luma) if luma > 0.85 => ThemeMode::Light,
            _ => ThemeMode::Dark,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SyntaxColors {
    pub comment: Color,
    pub keyword: Color,
    pub string: Color,
    pub number: Color,
    pub function: Color,
    pub function_macro: Color,
    pub r#type: Color,
    pub variable_builtin: Color,
    pub variable_member: Color,
    pub module: Color,
    pub operator: Color,
    pub tag: Color,
    pub attribute: Color,
    pub label: Color,
    pub punctuation: Color,
    pub default_text: Color,
}

#[derive(Debug, Clone)]
pub struct DiffColors {
    pub added_bg: Color,
    pub added_gutter_bg: Color,
    pub added_gutter_fg: Color,
    pub deleted_bg: Color,
    pub deleted_gutter_bg: Color,
    pub deleted_gutter_fg: Color,
    pub context_bg: Color,
    pub empty_placeholder_fg: Color,
    /// Word-level highlight for added text.
    pub added_word_bg: Color,
    /// Word-level highlight for deleted text.
    pub deleted_word_bg: Color,
}

#[derive(Debug, Clone)]
pub struct UiColors {
    pub border_focused: Color,
    pub border_unfocused: Color,
    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_muted: Color,
    pub line_number: Color,
    pub bg: Color,
    pub footer_branch_bg: Color,
    pub footer_branch_fg: Color,
    pub status_added: Color,
    pub status_modified: Color,
    pub status_deleted: Color,
    pub stats_added: Color,
    pub stats_removed: Color,
    pub selection_bg: Color,
    pub selection_fg: Color,
    pub highlight: Color,
    pub viewed: Color,
    pub watching: Color,
    pub search_match_bg: Color,
    pub search_match_fg: Color,
    pub search_current_bg: Color,
    pub search_current_fg: Color,
}

#[derive(Debug, Clone)]
pub struct Theme {
    #[allow(dead_code)]
    pub mode: ThemeMode,
    pub syntax: SyntaxColors,
    pub diff: DiffColors,
    pub ui: UiColors,
}

impl Theme {
    /// Builds the dark default palette.
    ///
    /// This is a deliberate product choice, not a theme preset: callers should
    /// usually go through `from_mode` or `get` so the UI follows the terminal.
    pub fn dark() -> Self {
        Self {
            mode: ThemeMode::Dark,
            syntax: SyntaxColors {
                comment: Color::Rgb(106, 115, 125),
                keyword: Color::Rgb(255, 123, 114),
                string: Color::Rgb(165, 214, 255),
                number: Color::Rgb(121, 192, 255),
                function: Color::Rgb(210, 168, 255),
                function_macro: Color::Rgb(86, 182, 194),
                r#type: Color::Rgb(255, 203, 107),
                variable_builtin: Color::Rgb(255, 123, 114),
                variable_member: Color::Rgb(121, 192, 255),
                module: Color::Rgb(230, 192, 123),
                operator: Color::Rgb(255, 123, 114),
                tag: Color::Rgb(126, 231, 135),
                attribute: Color::Rgb(121, 192, 255),
                label: Color::Rgb(255, 160, 122),
                punctuation: Color::Rgb(200, 200, 200),
                default_text: Color::Rgb(230, 230, 230),
            },
            diff: DiffColors {
                added_bg: Color::Rgb(35, 50, 40),
                added_gutter_bg: Color::Rgb(40, 80, 50),
                added_gutter_fg: Color::Rgb(140, 200, 160),
                deleted_bg: Color::Rgb(50, 35, 35),
                deleted_gutter_bg: Color::Rgb(80, 40, 40),
                deleted_gutter_fg: Color::Rgb(200, 140, 140),
                context_bg: Color::Rgb(40, 40, 50),
                empty_placeholder_fg: Color::Rgb(55, 60, 70),
                added_word_bg: Color::Rgb(40, 85, 55),
                deleted_word_bg: Color::Rgb(100, 50, 50),
            },
            ui: UiColors {
                border_focused: Color::Cyan,
                border_unfocused: Color::DarkGray,
                text_primary: Color::Rgb(230, 230, 230),
                text_secondary: Color::Rgb(200, 200, 200),
                text_muted: Color::Rgb(140, 140, 160),
                line_number: Color::DarkGray,
                bg: Color::Reset,
                footer_branch_bg: Color::Rgb(50, 50, 70),
                footer_branch_fg: Color::Rgb(180, 180, 220),
                status_added: Color::Green,
                status_modified: Color::Yellow,
                status_deleted: Color::Red,
                stats_added: Color::Rgb(80, 200, 120),
                stats_removed: Color::Rgb(240, 80, 80),
                selection_bg: Color::Cyan,
                selection_fg: Color::Black,
                highlight: Color::Yellow,
                viewed: Color::Green,
                watching: Color::Yellow,
                search_match_bg: Color::Rgb(100, 80, 20),
                search_match_fg: Color::Rgb(255, 220, 120),
                search_current_bg: Color::Rgb(255, 165, 0),
                search_current_fg: Color::Black,
            },
        }
    }

    /// Builds the light default palette for bright terminal backgrounds.
    pub fn light() -> Self {
        Self {
            mode: ThemeMode::Light,
            syntax: SyntaxColors {
                comment: Color::Rgb(106, 115, 125),
                keyword: Color::Rgb(207, 34, 46),
                string: Color::Rgb(10, 48, 105),
                number: Color::Rgb(5, 80, 174),
                function: Color::Rgb(130, 80, 223),
                function_macro: Color::Rgb(17, 99, 41),
                r#type: Color::Rgb(149, 56, 0),
                variable_builtin: Color::Rgb(207, 34, 46),
                variable_member: Color::Rgb(5, 80, 174),
                module: Color::Rgb(149, 56, 0),
                operator: Color::Rgb(207, 34, 46),
                tag: Color::Rgb(17, 99, 41),
                attribute: Color::Rgb(5, 80, 174),
                label: Color::Rgb(191, 87, 0),
                punctuation: Color::Rgb(87, 96, 106),
                default_text: Color::Rgb(36, 41, 47),
            },
            diff: DiffColors {
                added_bg: Color::Rgb(230, 255, 237),
                added_gutter_bg: Color::Rgb(180, 240, 200),
                added_gutter_fg: Color::Rgb(36, 100, 60),
                deleted_bg: Color::Rgb(255, 245, 243),
                deleted_gutter_bg: Color::Rgb(255, 210, 205),
                deleted_gutter_fg: Color::Rgb(140, 60, 60),
                context_bg: Color::Rgb(246, 248, 250),
                empty_placeholder_fg: Color::Rgb(200, 205, 212),
                added_word_bg: Color::Rgb(171, 242, 188),
                deleted_word_bg: Color::Rgb(255, 184, 174),
            },
            ui: UiColors {
                border_focused: Color::Rgb(9, 105, 218),
                border_unfocused: Color::Rgb(208, 215, 222),
                text_primary: Color::Rgb(36, 41, 47),
                text_secondary: Color::Rgb(87, 96, 106),
                text_muted: Color::Rgb(140, 149, 159),
                line_number: Color::Rgb(140, 149, 159),
                bg: Color::Reset,
                footer_branch_bg: Color::Rgb(221, 244, 255),
                footer_branch_fg: Color::Rgb(9, 105, 218),
                status_added: Color::Rgb(26, 127, 55),
                status_modified: Color::Rgb(154, 103, 0),
                status_deleted: Color::Rgb(207, 34, 46),
                stats_added: Color::Rgb(26, 127, 55),
                stats_removed: Color::Rgb(207, 34, 46),
                selection_bg: Color::Rgb(9, 105, 218),
                selection_fg: Color::White,
                highlight: Color::Rgb(154, 103, 0),
                viewed: Color::Rgb(26, 127, 55),
                watching: Color::Rgb(154, 103, 0),
                search_match_bg: Color::Rgb(255, 235, 150),
                search_match_fg: Color::Black,
                search_current_bg: Color::Rgb(255, 140, 0),
                search_current_fg: Color::Black,
            },
        }
    }

    /// Converts a detected luminance mode into the only palette family we keep.
    ///
    /// # Example
    ///
    /// ```
    /// # #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    /// # enum ThemeMode { Dark, Light }
    /// # struct Theme { mode: ThemeMode }
    /// # impl Theme {
    /// #     fn dark() -> Self { Self { mode: ThemeMode::Dark } }
    /// #     fn light() -> Self { Self { mode: ThemeMode::Light } }
    /// #     fn from_mode(mode: ThemeMode) -> Self {
    /// #         match mode {
    /// #             ThemeMode::Dark => Self::dark(),
    /// #             ThemeMode::Light => Self::light(),
    /// #         }
    /// #     }
    /// # }
    /// assert_eq!(Theme::from_mode(ThemeMode::Dark).mode, ThemeMode::Dark);
    /// ```
    pub fn from_mode(mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::Dark => Self::dark(),
            ThemeMode::Light => Self::light(),
        }
    }

    pub fn from_choice(choice: ThemeChoice) -> Self {
        match choice {
            ThemeChoice::Auto | ThemeChoice::Default => Self::from_mode(ThemeMode::detect()),
            ThemeChoice::Tokyonight => Self::tokyonight(),
            ThemeChoice::Midnight => Self::midnight(),
        }
    }

    /// Builds a Tokyo Night-inspired palette without opening a general theme system.
    pub fn tokyonight() -> Self {
        Self {
            mode: ThemeMode::Dark,
            syntax: SyntaxColors {
                comment: Color::Rgb(86, 95, 137),
                keyword: Color::Rgb(187, 154, 247),
                string: Color::Rgb(158, 206, 106),
                number: Color::Rgb(255, 158, 100),
                function: Color::Rgb(122, 162, 247),
                function_macro: Color::Rgb(125, 207, 255),
                r#type: Color::Rgb(42, 195, 222),
                variable_builtin: Color::Rgb(247, 118, 142),
                variable_member: Color::Rgb(115, 218, 202),
                module: Color::Rgb(224, 175, 104),
                operator: Color::Rgb(137, 221, 255),
                tag: Color::Rgb(125, 207, 255),
                attribute: Color::Rgb(224, 175, 104),
                label: Color::Rgb(255, 158, 100),
                punctuation: Color::Rgb(169, 177, 214),
                default_text: Color::Rgb(192, 202, 245),
            },
            diff: DiffColors {
                added_bg: Color::Rgb(30, 58, 54),
                added_gutter_bg: Color::Rgb(35, 82, 70),
                added_gutter_fg: Color::Rgb(158, 206, 106),
                deleted_bg: Color::Rgb(65, 40, 58),
                deleted_gutter_bg: Color::Rgb(92, 48, 66),
                deleted_gutter_fg: Color::Rgb(247, 118, 142),
                context_bg: Color::Rgb(31, 35, 53),
                empty_placeholder_fg: Color::Rgb(49, 56, 87),
                added_word_bg: Color::Rgb(45, 91, 73),
                deleted_word_bg: Color::Rgb(120, 54, 72),
            },
            ui: UiColors {
                border_focused: Color::Rgb(122, 162, 247),
                border_unfocused: Color::Rgb(65, 72, 104),
                text_primary: Color::Rgb(192, 202, 245),
                text_secondary: Color::Rgb(169, 177, 214),
                text_muted: Color::Rgb(86, 95, 137),
                line_number: Color::Rgb(65, 72, 104),
                bg: Color::Reset,
                footer_branch_bg: Color::Rgb(36, 40, 59),
                footer_branch_fg: Color::Rgb(122, 162, 247),
                status_added: Color::Rgb(158, 206, 106),
                status_modified: Color::Rgb(224, 175, 104),
                status_deleted: Color::Rgb(247, 118, 142),
                stats_added: Color::Rgb(158, 206, 106),
                stats_removed: Color::Rgb(247, 118, 142),
                selection_bg: Color::Rgb(122, 162, 247),
                selection_fg: Color::Rgb(26, 27, 38),
                highlight: Color::Rgb(224, 175, 104),
                viewed: Color::Rgb(158, 206, 106),
                watching: Color::Rgb(224, 175, 104),
                search_match_bg: Color::Rgb(86, 69, 45),
                search_match_fg: Color::Rgb(255, 213, 128),
                search_current_bg: Color::Rgb(255, 158, 100),
                search_current_fg: Color::Rgb(26, 27, 38),
            },
        }
    }

    /// Builds a deeper dark palette for terminals where bright blues feel loud.
    pub fn midnight() -> Self {
        Self {
            mode: ThemeMode::Dark,
            syntax: SyntaxColors {
                comment: Color::Rgb(92, 104, 132),
                keyword: Color::Rgb(178, 140, 255),
                string: Color::Rgb(118, 214, 166),
                number: Color::Rgb(247, 184, 106),
                function: Color::Rgb(111, 178, 255),
                function_macro: Color::Rgb(93, 220, 217),
                r#type: Color::Rgb(132, 214, 247),
                variable_builtin: Color::Rgb(255, 121, 152),
                variable_member: Color::Rgb(143, 213, 255),
                module: Color::Rgb(226, 196, 132),
                operator: Color::Rgb(137, 221, 255),
                tag: Color::Rgb(93, 220, 217),
                attribute: Color::Rgb(226, 196, 132),
                label: Color::Rgb(247, 184, 106),
                punctuation: Color::Rgb(174, 184, 208),
                default_text: Color::Rgb(221, 226, 240),
            },
            diff: DiffColors {
                added_bg: Color::Rgb(18, 45, 39),
                added_gutter_bg: Color::Rgb(23, 69, 55),
                added_gutter_fg: Color::Rgb(118, 214, 166),
                deleted_bg: Color::Rgb(48, 25, 38),
                deleted_gutter_bg: Color::Rgb(75, 32, 48),
                deleted_gutter_fg: Color::Rgb(255, 121, 152),
                context_bg: Color::Rgb(17, 23, 36),
                empty_placeholder_fg: Color::Rgb(36, 45, 63),
                added_word_bg: Color::Rgb(29, 86, 65),
                deleted_word_bg: Color::Rgb(97, 39, 57),
            },
            ui: UiColors {
                border_focused: Color::Rgb(93, 220, 217),
                border_unfocused: Color::Rgb(55, 67, 90),
                text_primary: Color::Rgb(221, 226, 240),
                text_secondary: Color::Rgb(174, 184, 208),
                text_muted: Color::Rgb(104, 116, 145),
                line_number: Color::Rgb(78, 90, 116),
                bg: Color::Reset,
                footer_branch_bg: Color::Rgb(20, 29, 45),
                footer_branch_fg: Color::Rgb(143, 213, 255),
                status_added: Color::Rgb(118, 214, 166),
                status_modified: Color::Rgb(247, 184, 106),
                status_deleted: Color::Rgb(255, 121, 152),
                stats_added: Color::Rgb(118, 214, 166),
                stats_removed: Color::Rgb(255, 121, 152),
                selection_bg: Color::Rgb(93, 220, 217),
                selection_fg: Color::Rgb(8, 13, 23),
                highlight: Color::Rgb(247, 184, 106),
                viewed: Color::Rgb(118, 214, 166),
                watching: Color::Rgb(247, 184, 106),
                search_match_bg: Color::Rgb(74, 56, 28),
                search_match_fg: Color::Rgb(255, 222, 150),
                search_current_bg: Color::Rgb(247, 184, 106),
                search_current_fg: Color::Rgb(8, 13, 23),
            },
        }
    }
}

/// Initializes the palette for this terminal session.
pub fn init(choice: ThemeChoice) {
    let _ = THEME.set(Theme::from_choice(choice));
}

pub fn get() -> &'static Theme {
    THEME.get_or_init(|| Theme::from_mode(ThemeMode::detect()))
}
