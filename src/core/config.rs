use std::alloc::Layout;
use tui::layout::Constraint;
use tui::style::{Color, Modifier};

pub struct Config {
    pub name_config: NameConfig,
    pub font_config: FontConfig,
    pub layout_config: LayoutConfig,
    pub text_config: TextConfig,
}

impl Config {
    pub fn new() -> Config {
        Config {
            name_config: NameConfig::new(),
            font_config: FontConfig::new(),
            layout_config: LayoutConfig::new(),
            text_config: TextConfig::new(),
        }
    }
}

pub struct FontConfig {
    pub border: Color,
    pub selected_border: Color,
    pub highlight_fg: Color,
    pub highlight_bg: Color,
    pub highlight_modifier: Modifier,
    pub text_fg: Color,
    pub first_letter_fg: Color,
    pub input_fg: Color,
}

impl FontConfig {
    pub fn new() -> FontConfig {
        FontConfig {
            border: Color::DarkGray,
            selected_border: Color::White,
            highlight_fg: Color::Red,
            highlight_bg: Color::Gray,
            highlight_modifier: Modifier::BOLD,
            text_fg: Color::White,
            first_letter_fg: Color::Red,
            input_fg: Color::Yellow,
        }
    }
}

pub struct NameConfig {
    pub app_title: String,
    pub namespace: String,
    pub command: String,
    pub tabs_title: Vec<String>,
    pub namespaces_title: String,
    pub commands_title: String,
    pub tags_title: String,
    pub highlight_symbol: String,
    pub confirm_title: String,
    pub add_namespace_title: String,
    pub add_command_title: String,
}

impl NameConfig {
    pub fn new() -> NameConfig {
        NameConfig {
            app_title: "Command Manager".to_string(),
            namespace: "namespace".to_string(),
            command: "command".to_string(),
            tabs_title: vec![
                "Tab 1".to_string(),
                "Tab 2".to_string(),
                "Tab 3".to_string(),
            ],
            namespaces_title: "Namespaces".to_string(),
            commands_title: "Commands".to_string(),
            tags_title: "Tags".to_string(),
            highlight_symbol: "⟩".to_string(),
            confirm_title: "Confirm".to_string(),
            add_namespace_title: "Type the namespace name".to_string(),
            add_command_title: "Type the command script".to_string(),
        }
    }
}

pub struct LayoutConfig {
    pub app_block: Vec<Constraint>,
    pub main_block: Vec<Constraint>,
    pub lists_block: Vec<Constraint>,
    pub highlight_border_fg: Color,
}

impl LayoutConfig {
    pub fn new() -> LayoutConfig {
        LayoutConfig {
            app_block: vec![Constraint::Length(3), Constraint::Min(0)],
            main_block: vec![
                Constraint::Percentage(50),
                Constraint::Percentage(20),
                Constraint::Percentage(30),
            ],
            lists_block: vec![
                Constraint::Percentage(15),
                Constraint::Percentage(75),
                Constraint::Percentage(10),
            ],
            highlight_border_fg: Color::Green,
        }
    }
}

pub struct TextConfig {
    pub confirm_command: String,
    pub confirm_command_answer: String,
}

impl TextConfig {
    pub fn new() -> TextConfig {
        TextConfig {
            confirm_command: "Execute the selected command ? (press Esc to cancel)".to_string(),
            confirm_command_answer: "Press Enter".to_string(),
        }
    }
}
