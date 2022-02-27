use crate::app::state::{StatefulList, TabsState};
use crate::core::config::Config;

use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Tabs};
use tui::Frame;

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

    pub fn create_tabs(&self, items: &Rc<RefCell<TabsState>>) -> Tabs {
        let tabs_ref = items.as_ref().borrow();

        let titles = tabs_ref
            .titles
            .iter()
            .map(|t| {
                let (first, rest) = t.split_at(1);
                Spans::from(vec![
                    Span::styled(
                        first.to_string(),
                        Style::default().fg(self.config.font_config.first_letter_fg),
                    ),
                    Span::styled(
                        rest.to_string(),
                        Style::default().fg(self.config.font_config.text_fg),
                    ),
                ])
            })
            .collect::<Vec<Spans>>();

        Tabs::new(titles)
            .block(self.get_block("".to_string()))
            .style(self.get_border_style(tabs_ref.current_selected))
            .highlight_style(self.get_highlight_style())
            .select(tabs_ref.index)
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
                self.get_highlight_style(),
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

        let block = ui_builder
            .get_block(title)
            .style(ui_builder.get_border_style(true));

        let area = if let Some((percent_x, percent_y)) = rect_dimensions {
            self.get_centered_rect(percent_x, percent_y, rect)
        } else {
            self.get_centered_rect(70, 20, rect)
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

    pub fn get_centered_rect(&self, percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let split_y = (100 - percent_y) / 2;
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(split_y),
                    Constraint::Percentage(percent_y),
                    Constraint::Percentage(split_y),
                ]
                .as_ref(),
            )
            .split(r);

        let split_x = (100 - percent_x) / 2;
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(split_x),
                    Constraint::Percentage(percent_x),
                    Constraint::Percentage(split_x),
                ]
                .as_ref(),
            )
            .split(popup_layout[1])[1]
    }
}
