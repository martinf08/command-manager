use crate::app::state::StatefulList;
use crate::core::config::Config;

use std::cell::RefMut;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, List, ListItem};

pub struct UiBuilder {
    config: Config,
}

pub struct LayoutBuilder;

impl UiBuilder {
    pub fn new() -> Self {
        UiBuilder {
            config: Config::new(),
        }
    }

    pub fn create_list(&self, title: String, items: &RefMut<StatefulList<String>>) -> List {
        let list_item = items
            .items
            .iter()
            .filter(|item| !item.trim().is_empty())
            .map(|item| ListItem::new(item.clone()).style(Style::default().fg(Color::White)))
            .collect::<Vec<ListItem>>();

        List::new(list_item)
            .block(self.get_block(title))
            .style(self.get_border_style(items.current_selected))
            .highlight_style(self.get_highlight_style())
            .highlight_symbol(&*self.config.name_config.highlight_symbol)
    }

    pub fn get_border_style(&self, selected: bool) -> Style {
        if selected {
            return Style::default().fg(self.config.font_config.selected_border);
        }

        Style::default().fg(self.config.font_config.border)
    }

    pub fn get_block(&self, title: String) -> Block {
        Block::default().title(title).borders(Borders::ALL)
    }

    pub fn get_highlight_style(&self) -> Style {
        Style::default()
            .add_modifier(self.config.font_config.highlight_modifier)
            .fg(self.config.font_config.highlight_fg)
            .bg(self.config.font_config.highlight_bg)
    }
}

impl LayoutBuilder {
    pub fn create(constraints: Vec<Constraint>, direction: Direction) -> Layout {
        Layout::default()
            .direction(direction)
            .constraints(constraints)
    }
}
