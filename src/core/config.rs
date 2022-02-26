use tui::style::{Color, Modifier};

pub struct Config {
    pub name: String,
    pub tabs: Vec<String>,
    pub color_config: ColorConfig,
    pub namespace_title: String,
    pub highlight_symbol: String,
}

pub struct ColorConfig {
    pub border: Color,
    pub selected_border: Color,
    pub highlight_fg: Color,
    pub highlight_bg: Color,
    pub highlight_modifier: Modifier,
}

impl ColorConfig {
    pub fn new() -> ColorConfig {
        ColorConfig {
            border: Color::DarkGray,
            selected_border: Color::White,
            highlight_fg: Color::Red,
            highlight_bg: Color::Gray,
            highlight_modifier: Modifier::BOLD,
        }
    }
}

impl Config {
    pub fn new() -> Config {
        Config {
            name: "Command Manager".to_string(),
            tabs: vec![
                "Tab 1".to_string(),
                "Tab 2".to_string(),
                "Tab 3".to_string(),
            ],
            color_config: ColorConfig::new(),
            namespace_title: "Namespace".to_string(),
            highlight_symbol: "⟩".to_string(),
        }
    }
}
