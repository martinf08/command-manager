use crate::app::state::StatefulList;
use crate::core::config::Config;

use std::cell::RefMut;
use tui::backend::Backend;
use tui::Frame;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph};
use crate::ui::utils::{centered_rect, get_border_style_from_selected_status, get_highlight_style};

pub struct UiBuilder {
    config: Config,
}

pub struct LayoutBuilder {
    config: Config,
}

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

    pub fn get_confirm_command(&self, alignment: Alignment) -> Paragraph {

        let text = vec![
            Spans::from(Span::styled(
                self.config.text_config.confirm_command.clone(),
                Style::default().fg(self.config.font_config.text_fg),
            )),
            Spans::from(Span::raw("")),
            Spans::from(Span::styled(
                self.config.text_config.confirm_command_answer.clone(),
                self.get_highlight_style()
            )),
        ];

        Paragraph::new(text).alignment(alignment)
    }
}

impl LayoutBuilder {
    pub fn new() -> Self {
        LayoutBuilder {
            config: Config::new(),
        }
    }

    pub fn create(&self, constraints: Vec<Constraint>, direction: Direction) -> Layout {
        Layout::default()
            .direction(direction)
            .constraints(constraints)
    }

    pub fn get_popup_layout<B>(
        &self,
        title: String,
        f: &mut Frame<B>,
        rect: Rect,
        margin_ratio: Option<u8>,
        rect_dimensions: Option<(u16, u16)>,
    ) -> Vec<Rect>
        where
            B: Backend,
    {
        let ui_builder = UiBuilder::new();

        let block = ui_builder.get_block(title)
            .style(get_border_style_from_selected_status(true));

        let area = if let Some((percent_x, percent_y)) = rect_dimensions {
            centered_rect(percent_x, percent_y, rect)
        } else {
            centered_rect(70, 20, rect)
        };

        f.render_widget(Clear, area);
        f.render_widget(block, area);

        let layout = self.create(vec![Constraint::Percentage(100)], Direction::Horizontal);

        if margin_ratio.is_some() {
            let layout = layout.margin(area.height / margin_ratio.unwrap() as u16);

            return layout.split(area);
        }

        layout.split(area)
    }
}
