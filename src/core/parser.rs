use crate::app::add::{AddType, InputMode};
use crate::app::app::{App, CursorPosition, Mode, State, StatefulList};
use crate::db::{add_namespace, get_namespace, get_namespaces};
use crossterm::event::{KeyCode, KeyEvent};
use std::error::Error;

pub struct KeyParser;

pub type ParserResult = Result<Option<(String, String)>, Box<dyn Error>>;

impl KeyParser {
    pub fn parse_event(key_event: KeyEvent, app: &mut App) -> ParserResult {
        KeyParser::process_key_code(key_event.code, app)
    }

    fn process_key_code(key_code: KeyCode, app: &mut App) -> ParserResult {
        if key_code == KeyCode::Char('q')
            && (app.mode == Mode::Normal || app.mode == Mode::Delete) {
            KeyParser::quit(app)?;
            return Ok(None);
        }

        match app.tabs.index {
            0 => KeyParser::process_tab_0(key_code, app),
            1 => KeyParser::process_tab_1(key_code, app),
            2 => KeyParser::process_tab_2(key_code, app),
            _ => Ok(None),
        }
    }

    fn quit(app: &mut App) -> ParserResult {
        app.quit = true;
        Ok(None)
    }

    fn process_tab_0(key_code: KeyCode, app: &mut App) -> ParserResult {
        match app.mode {
            Mode::Normal => KeyParser::process_normal_mode(key_code, app),
            Mode::Add => KeyParser::process_add_mode(key_code, app),
            Mode::Delete => KeyParser::process_delete_mode(key_code, app),
        }
    }

    fn process_tab_1(key_code: KeyCode, app: &mut App) -> ParserResult {
        match key_code {
            KeyCode::Right => {
                app.tabs.next();
                Ok(None)
            }
            KeyCode::Left => {
                app.tabs.previous();
                Ok(None)
            }
            _ => Ok(None),
        }
    }

    fn process_tab_2(key_code: KeyCode, app: &mut App) -> ParserResult {
        match key_code {
            KeyCode::Right => {
                app.tabs.next();
                Ok(None)
            }
            KeyCode::Left => {
                app.tabs.previous();
                Ok(None)
            }
            _ => Ok(None),
        }
    }

    fn process_normal_mode(key_code: KeyCode, app: &mut App) -> ParserResult {
        match key_code {
            KeyCode::Right => KeyParser::move_right(app),
            KeyCode::Left => KeyParser::move_left(app),
            KeyCode::Down => KeyParser::move_down(app),
            KeyCode::Up => KeyParser::move_up(app),
            KeyCode::Enter => KeyParser::enter(app),
            KeyCode::Esc => KeyParser::esc(app),
            KeyCode::Char('a') => {
                app.change_mode(Mode::Add);
                Ok(None)
            }
            _ => Ok(None),
        }
    }

    fn process_add_mode(key_code: KeyCode, app: &mut App) -> ParserResult {
        match key_code {
            KeyCode::Esc => {
                app.change_mode(Mode::Normal);
                Ok(None)
            }
            _ => match &app.add.add_type {
                Some(t) => match app.add.input_mode {
                    Some(InputMode::Namespace) => KeyParser::process_add_namespace(key_code, app),
                    Some(InputMode::Command) => unimplemented!(),
                    None => unreachable!(),
                },
                None => match key_code {
                    KeyCode::Char('c') => {
                        app.add.add_type = Some(AddType::Command);
                        Ok(None)
                    }
                    KeyCode::Char('n') => {
                        app.add.add_type = Some(AddType::Namespace);
                        app.add.input_mode = Some(InputMode::Namespace);

                        Ok(None)
                    }
                    _ => Ok(None),
                },
            },
        }
    }

    fn process_delete_mode(key_code: KeyCode, app: &mut App) -> ParserResult {
        unimplemented!()
    }

    fn move_right(app: &mut App) -> ParserResult {
        match app.namespaces.state.selected() {
            Some(_) => {
                app.switch_selected_widgets_off();
                app.switch_selected_commands_tags_on()
            }
            None => app.tabs.next(),
        }

        Ok(None)
    }

    fn move_left(app: &mut App) -> ParserResult {
        match app.commands.state.selected() {
            Some(_) => {
                app.switch_selected_commands_tags_off();
                app.set_current_selected_namespace(true)
            }
            None => match app.namespaces.state.selected() {
                Some(_) => {
                    app.switch_selected_namespaces_off();
                    app.switch_selected_commands_tags_off();
                    app.set_current_selected_tab(true);
                }
                None => app.tabs.previous(),
            },
        }
        Ok(None)
    }

    fn move_down(app: &mut App) -> ParserResult {
        match app.commands.state.selected() {
            Some(_) => {
                app.commands.next();
                app.tags.next();
            }
            None => match app.namespaces.state.selected() {
                Some(_) => {
                    app.namespaces.next();
                    app.set_commands_tags_from_position(app.namespaces.current());
                }
                None => {
                    app.set_current_selected_tab(false);
                    app.switch_selected_namespaces_on();
                }
            },
        }

        Ok(None)
    }

    fn move_up(app: &mut App) -> ParserResult {
        match app.commands.state.selected() {
            Some(_) => {
                app.commands.previous();
                app.tags.previous();
            }
            None => {
                app.namespaces.previous();
                app.set_commands_tags_from_position(app.namespaces.current());
            }
        };

        Ok(None)
    }

    fn enter(app: &mut App) -> ParserResult {
        match app.commands.state.selected() {
            Some(_) => match app.show_command_confirmation {
                true => {
                    return Ok(Some((
                        app.commands.items[app.commands.current()].clone(),
                        app.tags.items[app.tags.current()].clone(),
                    )));
                }
                false => {
                    app.set_current_selected_commands_tags(false);
                    app.set_show_command_confirmation(true);
                }
            },
            None => match app.namespaces.state.selected() {
                Some(_) => {
                    app.set_current_selected_namespace(false);
                    app.switch_selected_commands_tags_on();
                }
                None => {}
            },
        }

        Ok(None)
    }

    fn esc(app: &mut App) -> ParserResult {
        match app.show_command_confirmation {
            true => {
                app.set_current_selected_commands_tags(true);
                app.set_show_command_confirmation(false);
            }
            false => {
                app.switch_selected_commands_tags_off();
                app.switch_selected_namespaces_off();
                app.set_current_selected_tab(true);
            }
        }

        Ok(None)
    }

    fn process_add_namespace(key_code: KeyCode, app: &mut App) -> ParserResult {
        match key_code {
            KeyCode::Esc => {
                app.add.add_type = None;
                app.mode = Mode::Normal;
                Ok(None)
            }
            KeyCode::Char(c) => {
                app.add.input.push(c);
                app.cursor_position.as_mut().unwrap().push_inc(c);
                Ok(None)
            }
            KeyCode::Backspace => {
                app.add.input.pop();
                app.cursor_position.as_mut().unwrap().pop_dec();
                Ok(None)
            }
            KeyCode::Enter => match app.add.add_type {
                Some(AddType::Namespace) => {
                    if app.add.input.is_empty() {
                        return Ok(None);
                    }

                    let existing_namespace = get_namespace(&app.add.input);

                    if let Ok(option) = existing_namespace {
                        if let Some(s) = option {
                            let message = format!("Namespace {} already exists", s);

                            app.add.error_message = Some(message);
                            return Ok(None);
                        }
                    }

                    add_namespace(&app.add.input);

                    app.add.input.clear();
                    app.add.add_type = None;
                    app.mode = Mode::Normal;

                    let namespaces = get_namespaces().expect("Failed to get namespaces");
                    app.namespaces = StatefulList::with_items(namespaces);
                    app.cursor_position = None;

                    Ok(None)
                }
                _ => Ok(None),
            },
            _ => Ok(None),
        }
    }
}
