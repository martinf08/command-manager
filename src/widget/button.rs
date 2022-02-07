use tui::buffer::Buffer;
use tui::layout::{Alignment, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::Widget;

pub struct Button<'a> {
    text: &'a str,
    style: Style,
    alignement: Alignment,
}

impl<'a> Button<'a> {
    pub fn new(text: &'a str) -> Self {
        Button {
            text,
            style: Style::default(),
            alignement: Alignment::Center,
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn alignment(mut self, alignement: Alignment) -> Self {
        self.alignement = alignement;
        self
    }
}

impl<'a> Widget for Button<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let first_letter = self.text.chars().next().unwrap().to_string();
        let rest = self.text.chars().skip(1).collect::<String>();

        let span_first_letter = Span::styled(
            first_letter,
            Style::default()
                .fg(Color::Red)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        );

        let span_rest = Span::styled(rest, Style::default().fg(Color::White));

        let x = (area.x.saturating_sub((self.text.len() / 2) as u16)) + (area.width as u16) / 2;
        let y = area.y + (area.height as u16) / 2;

        buf.set_spans(
            x,
            y,
            &Spans::from(vec![span_first_letter, span_rest]),
            area.width,
        );
    }
}
