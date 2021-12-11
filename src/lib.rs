pub mod app;
mod fixtures;
mod ui;

use crossterm::event;
use crossterm::event::{Event, KeyCode};
use std::io;
use std::time::Duration;
use tui::backend::Backend;
use tui::Terminal;

use crate::app::{App, State};
use crate::ui::ui;

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
) -> io::Result<Option<(String, String)>> {

    app.tabs.current_selected = true;

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(None),
                    KeyCode::Right => match app.folders.state.selected() {
                        Some(_) => {
                            app.folders.current_selected = false;
                            app.commands.current_selected = true;

                            app.commands.state.select(Some(0));
                        }
                        None => app.tabs.next(),
                    },
                    KeyCode::Left => match app.commands.state.selected() {
                        Some(_) => {
                            app.commands.current_selected = false;
                            app.folders.current_selected = true;

                            app.commands.state.select(None);
                        }
                        None => match app.folders.state.selected() {
                            Some(_) => {
                                app.folders.current_selected = false;
                                app.tabs.current_selected = true;

                                app.folders.state.select(None);
                                app.commands.set_list_position(0)
                            }
                            None => app.tabs.previous(),
                        },
                    },
                    KeyCode::Down => match app.commands.state.selected() {
                        Some(_) => app.commands.next(),
                        None => match app.folders.state.selected() {
                            Some(_) => {
                                app.folders.next();
                                app.commands.set_list_position(app.folders.current());
                            },
                            None => {
                                app.tabs.current_selected = false;
                                app.folders.current_selected = true;

                                app.folders.state.select(Some(0));
                                app.commands.set_list_position(app.folders.current());
                            }
                        },
                    },
                    KeyCode::Up => match app.commands.state.selected() {
                        Some(_) => app.commands.previous(),
                        None => {
                            app.folders.previous();
                            app.commands.set_list_position(app.folders.current());
                        }
                    },
                    KeyCode::Enter => match app.commands.state.selected() {
                        Some(_) => match app.show_command_confirmation {
                            true => {
                                return Ok(Option::from(
                                    app.commands.items[app.commands.index]
                                        [app.commands.state.selected().unwrap()]
                                    .clone(),
                                ));
                            }
                            false => {
                                app.commands.current_selected = false;
                                app.show_command_confirmation = true;
                            }
                        },
                        None => match app.folders.state.selected() {
                            Some(_) => {
                                app.folders.current_selected = false;
                                app.commands.current_selected = true;

                                app.commands.state.select(Some(0));
                            }
                            None => {}
                        },
                    },
                    KeyCode::Esc => match app.show_command_confirmation {
                        true => {
                            app.commands.current_selected = true;
                            app.show_command_confirmation = false;
                        }
                        false => {
                            app.commands.current_selected = false;
                            app.folders.current_selected = false;
                            app.tabs.current_selected = true;

                            app.commands.state.select(None);
                            app.folders.state.select(None);
                        }
                    },
                    _ => {}
                }
            }
        }
    }
}
