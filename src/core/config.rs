use tui::style::{Color, Modifier};

pub struct Config {
    pub names_config: NameConfig,
    pub font_config: FontConfig,
}

impl Config {
    pub fn new() -> Config {
        Config {
            names_config: NameConfig::new(),
            font_config: FontConfig::new(),
        }
    }
}

pub struct FontConfig {
    pub border: Color,
    pub selected_border: Color,
    pub highlight_fg: Color,
    pub highlight_bg: Color,
    pub highlight_modifier: Modifier,
}

impl FontConfig {
    pub fn new() -> FontConfig {
        FontConfig {
            border: Color::DarkGray,
            selected_border: Color::White,
            highlight_fg: Color::Red,
            highlight_bg: Color::Gray,
            highlight_modifier: Modifier::BOLD,
        }
    }
}

pub struct NameConfig {
    pub app_title: String,
    pub tabs_title: Vec<String>,
    pub namespaces_title: String,
    pub commands_title: String,
    pub tags_title: String,
    pub highlight_symbol: String,
}

impl NameConfig {
    pub fn new() -> NameConfig {
        NameConfig {
            app_title: "Command Manager".to_string(),
            tabs_title: vec![
                "Tab 1".to_string(),
                "Tab 2".to_string(),
                "Tab 3".to_string(),
            ],
            namespaces_title: "Namespaces".to_string(),
            commands_title: "Commands".to_string(),
            tags_title: "Tags".to_string(),
            highlight_symbol: "⟩".to_string(),
        }
    }
}
