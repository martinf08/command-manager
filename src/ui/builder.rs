use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, List, ListItem};
use crate::App;
use crate::app::state::StatefulList;
use crate::core::config::Config;

pub struct UiBuilder {
    config: Config,
}

impl UiBuilder {
    pub fn new() -> Self {
        UiBuilder { config: Config::new() }
    }

    pub fn create_list(&self, title: String, items: Ref<StatefulList<String>>, selected: bool) -> List {
        let items = items
            .items
            .iter()
            .filter(|item| !item.trim().is_empty())
            .map(|item| ListItem::new(item.clone()).style(Style::default().fg(Color::White)))
            .collect::<Vec<ListItem>>();

        List::new(items)
            .block(self.get_block(title))
            .style(self.get_border_style(selected))
            .highlight_style(self.get_highlight_style())
            .highlight_symbol(&*self.config.highlight_symbol)
    }

    pub fn get_border_style(&self, selected: bool) -> Style {
        if selected {
            return Style::default().fg(self.config.color_config.selected_border);
        }

        Style::default().fg(self.config.color_config.border)
    }

    pub fn get_block(&self, title: String) -> Block {
        Block::default().title(title).borders(Borders::ALL)
    }

    pub fn get_highlight_style(&self) -> Style {
        Style::default()
            .add_modifier(self.config.color_config.highlight_modifier)
            .fg(self.config.color_config.highlight_fg)
            .bg(self.config.color_config.highlight_bg)
    }
}