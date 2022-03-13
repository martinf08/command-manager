use crate::app::app::App;

use crate::app::event_state::{Confirm, EventType, SubMode, Tab};
use crate::app::input::CursorPosition;
use crate::ui::builder::{LayoutBuilder, UiBuilder};

use tui::backend::Backend;
use tui::layout::{Alignment, Direction, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Clear, Paragraph, Wrap};
use tui::Frame;

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let ui_builder = UiBuilder::new();
    let layout_builder = LayoutBuilder::new();

    let chunks = layout_builder
        .create(
            app.config.layout_config.app_block.clone(),
            Direction::Vertical,
        )
        .split(f.size());

    // Display tabs
    let tabs = ui_builder.create_tabs(&app.tabs);
    f.render_widget(tabs, chunks[0]);

    match app.event_state.get_tab() {
        Tab::Tab1 => draw_first_tab(f, chunks[1], app),
    };
}

fn draw_first_tab<B>(f: &mut Frame<B>, rect: Rect, app: &mut App)
where
    B: Backend,
{
    let ui_builder = UiBuilder::new();
    let layout_builder = LayoutBuilder::new();

    let main_block = layout_builder
        .create(
            app.config.layout_config.main_block.clone(),
            Direction::Vertical,
        )
        .split(rect);

    let lists_block = layout_builder
        .create(
            app.config.layout_config.lists_block.clone(),
            Direction::Horizontal,
        )
        .split(main_block[0]);

    display_lists(app, f, &lists_block);

    match app.event_state.get_sub_mode() {
        SubMode::Namespace => {
            let input = String::from_iter(
                app.inputs
                    .entry(app.config.name_config.namespace.to_string())
                    .or_default()
                    .clone(),
            );

            let p = ui_builder.create_highlighted_paragraph(
                app.config.name_config.add_namespace_title.clone(),
                input.clone(),
                Alignment::Left,
            );

            CursorPosition::set_cursor_position(app, f, lists_block[1], input);

            f.render_widget(Clear, lists_block[1]);
            f.render_widget(p, lists_block[1]);
        }
        SubMode::Command => match app.event_state.get_event_type() {
            EventType::Command => {
                let input = String::from_iter(
                    app.inputs
                        .entry(app.config.name_config.command.to_string())
                        .or_default()
                        .clone(),
                );

                let p = ui_builder.create_highlighted_paragraph(
                    app.config.name_config.add_command_title.clone(),
                    input.clone(),
                    Alignment::Left,
                );

                CursorPosition::set_cursor_position(app, f, lists_block[1], input);

                f.render_widget(Clear, lists_block[1]);
                f.render_widget(p, lists_block[1]);
            }
            EventType::Tag => {
                let input = String::from_iter(
                    app.inputs
                        .entry(app.config.name_config.tag.to_string())
                        .or_default()
                        .clone(),
                );

                let p = ui_builder.create_highlighted_paragraph(
                    app.config.name_config.add_tag_title.clone(),
                    input.clone(),
                    Alignment::Left,
                );

                if input.is_empty() {
                    app.cursor_position = None;
                }
                CursorPosition::set_cursor_position(app, f, lists_block[1], input);

                f.render_widget(Clear, lists_block[1]);
                f.render_widget(p, lists_block[1]);
            }
            _ => {}
        },
        _ => {}
    }

    // Confirm popup
    if app.event_state.get_confirm() == &Confirm::Display {
        match app.event_state.get_event_type() {
            EventType::Command => {
                let input = String::from_iter(
                    app.inputs
                        .entry(app.config.name_config.tag.to_string())
                        .or_default()
                        .clone(),
                );

                let p = ui_builder.create_highlighted_paragraph(
                    app.config.name_config.add_tag_title.clone(),
                    input.clone(),
                    Alignment::Left,
                );

                CursorPosition::set_cursor_position(app, f, lists_block[1], input);

                f.render_widget(Clear, lists_block[1]);
                f.render_widget(p, lists_block[1]);
            }
            _ => {
                let popup_rects = layout_builder.get_popup_rects(
                    app.config.name_config.confirm_title.clone(),
                    f,
                    lists_block[1],
                    Some(3),
                    None,
                );

                let p = ui_builder.get_confirm_command(Alignment::Center);

                f.render_widget(p, popup_rects[0]);
            }
        }
    }

    //Command details
    let commands = app.commands.as_ref().borrow_mut();
    let mut command_text = "\n".to_string();
    if commands.state.selected().is_some() && commands.items.len() > 0 {
        command_text.push_str(&*commands.items[commands.state.selected().unwrap()].clone());
    }
    drop(commands);

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

fn display_lists(app: &mut App, f: &mut Frame<impl Backend>, lists_block: &Vec<Rect>) {
    let ui_builder = UiBuilder::new();

    let list = vec![
        (
            app.namespaces.as_ref().borrow_mut(),
            app.config.name_config.namespaces_title.to_string(),
        ),
        (
            app.commands.as_ref().borrow_mut(),
            app.config.name_config.commands_title.to_string(),
        ),
        (
            app.tags.as_ref().borrow_mut(),
            app.config.name_config.tags_title.to_string(),
        ),
    ];

    list.into_iter()
        .enumerate()
        .for_each(|(i, (mut list, title))| {
            let item_list = ui_builder.create_list(title.clone(), &list);
            f.render_stateful_widget(item_list, lists_block[i], &mut list.state);
        });
}
