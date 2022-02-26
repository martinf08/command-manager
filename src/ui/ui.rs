use crate::app::app::App;
use std::borrow::Borrow;

use crate::app::event_state::{Confirm, Mode, Tab};
use crate::core::config::Config;
use crate::ui::builder::{LayoutBuilder, UiBuilder};
use crate::ui::utils::{
    get_border_style_from_selected_status, get_highlight_style, get_popup_layout,
    set_cursor_position,
};
use crate::widget::button::Button;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph, Tabs, Wrap};
use tui::Frame;

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let config = Config::new();

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
        .block(Block::default().borders(Borders::ALL))
        .select(app.tabs.index)
        .style(get_border_style_from_selected_status(
            app.tabs.current_selected,
        ))
        .highlight_style(get_highlight_style());

    f.render_widget(tabs, chunks[0]);
    match app.event_state.get_tab() {
        Tab::Tab1 => draw_first_tab(f, chunks[1], app),
        Tab::Tab2 => draw_second_tab(f, chunks[1], app),
        Tab::Tab3 => draw_second_tab(f, chunks[1], app),
    };
}

fn draw_first_tab<B>(f: &mut Frame<B>, rect: Rect, app: &mut App)
where
    B: Backend,
{
    let config = Config::new();
    let builder = UiBuilder::new();

    let full_block = LayoutBuilder::create(vec![50, 20, 30], Direction::Vertical).split(rect);
    let middle_block =
        LayoutBuilder::create(vec![15, 75, 10], Direction::Horizontal).split(full_block[0]);

    // Display namespaces at left block
    let mut namespaces = app.namespaces.as_ref().borrow_mut();
    let namespaces_list = builder.create_list(config.names_config.namespaces_title, &namespaces);
    f.render_stateful_widget(namespaces_list, middle_block[0], &mut namespaces.state);

    //Display commands at middle block
    let mut commands = app.commands.as_ref().borrow_mut();
    let commands_list = builder.create_list(config.names_config.commands_title, &commands);
    f.render_stateful_widget(commands_list, middle_block[1], &mut commands.state);

    //Display tags at right block
    let mut tags = app.tags.as_ref().borrow_mut();
    let tags_list = builder.create_list(config.names_config.tags_title, &tags);

    f.render_stateful_widget(tags_list, middle_block[2], &mut tags.state);

    if app.event_state.get_confirm() == &Confirm::Display {
        let layout = get_popup_layout("Confirm".to_string(), f, middle_block[1], Some(3), None);

        let text = vec![
            Spans::from(Span::styled(
                "Execute command ?", //Todo config
                Style::default().fg(Color::White),
            )),
            Spans::from(Span::raw("")),
            Spans::from(Span::styled(
                "Yes", //Todo config
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Red)
                    .bg(Color::Gray),
            )),
        ];

        let p = Paragraph::new(text).alignment(Alignment::Center);

        f.render_widget(p, layout[0]);
    }

    let mut command_text = "\n".to_string();
    if commands.state.selected().is_some() && commands.items.len() > 0 {
        command_text.push_str(&*commands.items[commands.state.selected().unwrap()].clone());
    }

    // match *app.get_mode() {
    //     Mode::Add => match &app.add.add_type {
    //         Some(t) => match t {
    //             AddType::Command => match app.add.input_mode {
    //                 Some(InputMode::Command) | Some(InputMode::Tag) => {
    //                     if app.namespaces.state.selected().is_some() {
    //                         if app.add.input_mode == Some(InputMode::Command) {
    //                             command_text.push_str("Type the command");
    //                         } else {
    //                             command_text.push_str("Type the tag");
    //                         }
    //                         display_add_input_area(app, f, chunks[1])
    //                     }
    //                 }
    //                 _ => (),
    //             },
    //             AddType::Namespace => {
    //                 if let Some(InputMode::Namespace) = app.add.input_mode {
    //                     display_add_input_area(app, f, chunks[1])
    //                 }
    //             }
    //         },
    //         None => {
    //             display_add_type_selector(f, chunks[1]);
    //             command_text =
    //                 "Caution: Namespace must be selected before adding a command.".to_string();
    //         }
    //     },
    //     Mode::Delete => {
    //         if app.show_delete_confirmation
    //             && (app.commands.state.selected().is_some()
    //                 || app.namespaces.state.selected().is_some())
    //         {
    //             let layout = get_popup_layout("Confirm".to_string(), f, chunks[1], Some(3), None);
    //
    //             let text = vec![
    //                 Spans::from(Span::styled(
    //                     app.confirmation_popup.message,
    //                     Style::default().fg(Color::White),
    //                 )),
    //                 Spans::from(Span::raw("")),
    //                 Spans::from(Span::styled(
    //                     app.confirmation_popup.confirm,
    //                     Style::default()
    //                         .add_modifier(Modifier::BOLD)
    //                         .fg(Color::Red)
    //                         .bg(Color::Gray),
    //                 )),
    //             ];
    //
    //             let p = Paragraph::new(text).alignment(Alignment::Center);
    //
    //             f.render_widget(p, layout[0]);
    //         }
    //     }
    //     _ => {}
    // }

    let detail_command_paragraph = Paragraph::new(command_text)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Command details")
                .style(Style::default().fg(Color::White)),
        );

    f.render_widget(detail_command_paragraph, full_block[1]);
}

// fn display_add_input_area(app: &mut App, f: &mut Frame<impl Backend>, chunk: Rect) {
//     let title = match &app.add.add_type {
//         Some(t) => match t {
//             AddType::Command => "Command".to_string(),
//             AddType::Namespace => "Namespace".to_string(),
//         },
//         None => "".to_string(),
//     };
//
//     let rects = get_popup_layout(title, f, chunk, None, Some((100, 100)));
//
//     let chunks = Layout::default()
//         .direction(Direction::Horizontal)
//         .constraints([Constraint::Percentage(100)].as_ref())
//         .margin(1)
//         .split(rects[0]);
//
//     let p = Paragraph::new(app.add.input.clone())
//         .block(Block::default().style(Style::default().fg(Color::White)))
//         .style(Style::default().fg(Color::Yellow))
//         .wrap(Wrap { trim: true });
//
//     set_cursor_position(app, f, chunks[0], app.add.input.clone());
//
//     f.render_widget(p, chunks[0]);
// }

fn display_add_type_selector(f: &mut Frame<impl Backend>, rect: Rect) {
    let rects = get_popup_layout("Element to add".to_string(), f, rect, Some(3), None);

    let layout = Layout::default()
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .direction(Direction::Horizontal)
        .split(rects[0]);

    let command = Button::new("Command")
        .style(Style::default().fg(Color::Red))
        .alignment(Alignment::Center);

    let namespace = Button::new("Namespace")
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center);

    f.render_widget(command, layout[0]);
    f.render_widget(namespace, layout[1]);
}

fn draw_second_tab<B>(f: &mut Frame<B>, rect: Rect, _app: &mut App)
where
    B: Backend,
{
    let bloc = Block::default().title("Inner 2").borders(Borders::ALL);

    f.render_widget(bloc, rect);
}
