use crate::App;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Tabs};
use tui::Frame;

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());

    let titles = app
        .tabs
        .titles
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(first, Style::default().fg(Color::Red)),
                Span::styled(rest, Style::default().fg(Color::White)),
            ])
        })
        .collect::<Vec<Spans>>();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .select(app.tabs.index)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Black)
                .bg(Color::Cyan),
        );

    f.render_widget(tabs, chunks[0]);
    match app.tabs.index {
        0 => draw_first_tab(f, chunks[1], app),
        1 => draw_second_tab(f, chunks[1], app),
        2 => draw_second_tab(f, chunks[1], app),
        _ => unreachable!(),
    };
}

fn draw_first_tab<B>(f: &mut Frame<B>, rect: Rect, app: &mut App)
where
    B: Backend,
{
    let sub_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(rect);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(15),
                Constraint::Percentage(75),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(sub_chunks[0]);

    let bloc = Block::default().title("Folders").borders(Borders::ALL);

    f.render_widget(bloc, chunks[0]);

    let items = app
        .folders
        .items
        .iter()
        .filter(|item| !item.trim().is_empty())
        .map(|item| ListItem::new(item.as_str()).style(Style::default().fg(Color::Yellow)))
        .collect::<Vec<ListItem>>();

    let list = List::new(items)
        .block(Block::default().title("Folders").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>");

    f.render_stateful_widget(list, chunks[0], &mut app.folders.state);

    let commands: &Vec<(String, String)> = app.commands.items.get(app.folders.current()).unwrap();
    let commands_items = commands
        .iter()
        .map(|(key, _value)| ListItem::new(key.as_str()).style(Style::default().fg(Color::Yellow)))
        .collect::<Vec<ListItem>>();

    f.render_stateful_widget(
        List::new(commands_items)
            .block(Block::default().title("Commands").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>"),
        chunks[1],
        &mut app.commands.state,
    );

    let tags = commands
        .iter()
        .map(|(_key, value)| {
            ListItem::new(value.as_str()).style(Style::default().fg(Color::Yellow))
        })
        .collect::<Vec<ListItem>>();

    f.render_widget(
        List::new(tags)
            .block(Block::default().title("Tags").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>"),
        chunks[2],
    );

    if app.show_command_confirmation {
        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Yellow));

        let area = centered_rect(70, 20, chunks[1]);

        f.render_widget(Clear, area);
        f.render_widget(block, area);

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)].as_ref())
            .margin(area.height / 3)
            .split(area);

        let text = vec![
            Spans::from(Span::styled(
                app.confirmation_popup.message,
                Style::default().fg(Color::White),
            )),
            Spans::from(Span::raw("")),
            Spans::from(Span::styled(
                app.confirmation_popup.confirm,
                Style::default()
                    .add_modifier(Modifier::BOLD | Modifier::RAPID_BLINK)
                    .fg(Color::Black)
                    .bg(Color::Green),
            )),
        ];

        let p = Paragraph::new(text).alignment(Alignment::Center);

        f.render_widget(p, layout[0]);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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

fn draw_second_tab<B>(f: &mut Frame<B>, rect: Rect, _app: &mut App)
where
    B: Backend,
{
    let bloc = Block::default().title("Inner 2").borders(Borders::ALL);

    f.render_widget(bloc, rect);
}
