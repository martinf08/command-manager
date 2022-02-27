use crate::app::app::App;

use crate::app::event_state::{Confirm, Mode, Tab};
use crate::core::config::Config;
use crate::ui::builder::{LayoutBuilder, UiBuilder};
use crate::ui::utils::set_cursor_position;

use crate::widget::button::Button;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph, Tabs, Wrap};
use tui::Frame;

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let config = Config::new();
    let ui_builder = UiBuilder::new();
    let layout_builder = LayoutBuilder::new();

    let chunks = layout_builder
        .create(config.layout_config.app_block, Direction::Vertical)
        .split(f.size());

    // Display tabs
    let tabs = ui_builder.create_tabs(&app.tabs);
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
    let ui_builder = UiBuilder::new();
    let layout_builder = LayoutBuilder::new();

    let main_block = layout_builder
        .create(config.layout_config.main_block, Direction::Vertical)
        .split(rect);
    let lists_block = layout_builder
        .create(config.layout_config.lists_block, Direction::Horizontal)
        .split(main_block[0]);

    // Display namespaces at left block
    let mut namespaces = app.namespaces.as_ref().borrow_mut();
    let namespaces_list = ui_builder.create_list(config.name_config.namespaces_title, &namespaces);
    f.render_stateful_widget(namespaces_list, lists_block[0], &mut namespaces.state);

    //Display commands at middle block
    let mut commands = app.commands.as_ref().borrow_mut();
    let commands_list = ui_builder.create_list(config.name_config.commands_title, &commands);
    f.render_stateful_widget(commands_list, lists_block[1], &mut commands.state);

    //Display tags at right block
    let mut tags = app.tags.as_ref().borrow_mut();
    let tags_list = ui_builder.create_list(config.name_config.tags_title, &tags);
    f.render_stateful_widget(tags_list, lists_block[2], &mut tags.state);

    if app.event_state.get_confirm() == &Confirm::Display {
        let layout = layout_builder.get_popup_layout(
            config.name_config.confirm_title,
            f,
            lists_block[1],
            Some(3),
            None,
        );

        let p = ui_builder.get_confirm_command(Alignment::Center);

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

    f.render_widget(detail_command_paragraph, main_block[1]);
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

// fn display_add_type_selector(f: &mut Frame<impl Backend>, rect: Rect) {
//     let rects = get_popup_layout("Element to add".to_string(), f, rect, Some(3), None);
//
//     let layout = Layout::default()
//         .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
//         .direction(Direction::Horizontal)
//         .split(rects[0]);
//
//     let command = Button::new("Command")
//         .style(Style::default().fg(Color::Red))
//         .alignment(Alignment::Center);
//
//     let namespace = Button::new("Namespace")
//         .style(Style::default().fg(Color::White))
//         .alignment(Alignment::Center);
//
//     f.render_widget(command, layout[0]);
//     f.render_widget(namespace, layout[1]);
// }

fn draw_second_tab<B>(f: &mut Frame<B>, rect: Rect, _app: &mut App)
where
    B: Backend,
{
    let bloc = Block::default().title("Inner 2").borders(Borders::ALL);

    f.render_widget(bloc, rect);
}
