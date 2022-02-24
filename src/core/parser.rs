use crate::app::add::{AddType, InputMode};
use crate::app::app::{App, State, StatefulList};
use crate::app::event_state::{Confirm, Mode, Tab};
use crossterm::event::{KeyCode, KeyEvent};
use std::error::Error;
use crate::app::state::State;

pub struct KeyParser;

pub type ParserResult = Result<Option<(String, String)>, Box<dyn Error>>;

impl KeyParser {
    pub fn parse_event(key_event: KeyEvent, app: &mut App) -> ParserResult {
        KeyParser::process_key_code(key_event.code, app)
    }

    fn process_key_code(key_code: KeyCode, app: &mut App) -> ParserResult {
        if key_code == KeyCode::Char('q') && (app.mode == Mode::Normal || app.mode == Mode::Delete)
        {
            KeyParser::quit(app)?;
            return Ok(None);
        }

        match app.event_state.get_tab() {
            Tab::Tab1 => KeyParser::process_tab_1(key_code, app),
            // Tab::Tab2 => KeyParser::process_tab_2(key_code, app),
            // Tab::Tab3 => KeyParser::process_tab_3(key_code, app),
            _ => Ok(None),
        }
    }

    fn process_tab_1(key_code: KeyCode, app: &mut App) -> ParserResult {
        match app.event_state.get_mode() {
            Mode::Normal => KeyParser::process_normal_mode(key_code, app),
            // Mode::Add => KeyParser::process_add_mode(key_code, app),
            // Mode::Delete => KeyParser::process_delete_mode(key_code, app),
            _ => Ok(None), //Todo remove
        }
    }

    fn process_tab_2(key_code: KeyCode, app: &mut App) -> ParserResult {
        match key_code {
            KeyCode::Right | KeyCode::Char('l') => {
                app.tabs.next();
                Ok(None)
            }
            KeyCode::Left | KeyCode::Char('h') => {
                app.tabs.previous();
                Ok(None)
            }
            _ => Ok(None),
        }
    }

    fn process_tab_3(key_code: KeyCode, app: &mut App) -> ParserResult {
        match key_code {
            KeyCode::Right | KeyCode::Char('l') => {
                app.tabs.next();
                Ok(None)
            }
            KeyCode::Left | KeyCode::Char('h') => {
                app.tabs.previous();
                Ok(None)
            }
            _ => Ok(None),
        }
    }

    fn process_normal_mode(key_code: KeyCode, app: &mut App) -> ParserResult {
        match key_code {
            KeyCode::Right | KeyCode::Char('l') => KeyParser::move_right(app),
            KeyCode::Left | KeyCode::Char('h') => KeyParser::move_left(app),
            KeyCode::Down | KeyCode::Char('j') => KeyParser::move_down(app),
            KeyCode::Up | KeyCode::Char('k') => KeyParser::move_up(app),
            KeyCode::Enter => KeyParser::enter(app),
            KeyCode::Esc => KeyParser::esc(app),
            KeyCode::Char('a') => KeyParser::change_to_add_mode(app),
            KeyCode::Char('d') => KeyParser::change_to_delete_mode(app),
            _ => Ok(None),
        }
    }

    fn process_add_mode(key_code: KeyCode, app: &mut App) -> ParserResult {
        match key_code {
            KeyCode::Esc => {
                app.change_mode(Mode::Normal);
                app.add.add_type = None;
                Ok(None)
            }
            _ => match &app.add.add_type {
                Some(_) => match app.add.input_mode {
                    Some(InputMode::Namespace) => KeyParser::process_add_namespace(key_code, app),
                    Some(InputMode::Command) | Some(InputMode::Tag) => {
                        KeyParser::process_add_command(key_code, app)
                    }
                    _ => unreachable!(),
                },
                None => match key_code {
                    KeyCode::Char('c') => {
                        if app.namespaces.current_selected || app.commands.current_selected {
                            app.add.add_type = Some(AddType::Command);
                            app.add.input_mode = Some(InputMode::Command);
                        }

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
        if app.show_delete_confirmation {
            match key_code {
                KeyCode::Enter => {
                    if app.commands.current_selected {
                        app.db.delete_command(
                            app.commands.current_item(),
                            app.namespaces.current_item(),
                        )?;

                        let (commands, tags) = app
                            .db
                            .get_commands_and_tags(Some(app.namespaces.current_item().clone()))?;

                        app.commands = StatefulList::with_items(commands);
                        app.tags = StatefulList::with_items(tags);
                    } else if app.namespaces.current_selected {
                        app.db.delete_namespace(app.namespaces.current_item())?;

                        let namespaces = app.db.get_namespaces().expect("Failed to get namespaces");
                        app.namespaces = StatefulList::with_items(namespaces);

                        let (commands, tags) = app
                            .db
                            .get_commands_and_tags(Some(app.namespaces.current_item().clone()))?;
                        app.commands = StatefulList::with_items(commands);
                        app.tags = StatefulList::with_items(tags);
                    } else {
                        return Ok(None);
                    }

                    app.show_delete_confirmation = false;
                    app.change_mode(Mode::Normal);

                    return Ok(None);
                }
                KeyCode::Esc => {
                    app.show_delete_confirmation = false;
                    app.change_mode(Mode::Normal);

                    return Ok(None);
                }
                _ => {}
            }
        }

        Ok(None)
    }

    fn move_right(app: &mut App) -> ParserResult {
        match app.namespaces.state.selected() {
            Some(_) => {
                app.namespaces.current_selected = false;

                app.commands.current_selected = true;
                app.tags.current_selected = true;

                app.commands.state.select(Some(0));
                app.tags.state.select(Some(0));
            }
            None => app.tabs.next(),
        }

        Ok(None)
    }

    fn move_left(app: &mut App) -> ParserResult {
        match app.commands.state.selected() {
            Some(_) => {
                app.namespaces.current_selected = true;

                app.commands.current_selected = false;
                app.tags.current_selected = false;

                app.commands.unselect();
                app.tags.unselect();
            }
            None => match app.namespaces.state.selected() {
                Some(_) => {
                    app.namespaces.current_selected = false;
                    app.commands.current_selected = false;
                    app.tags.current_selected = false;

                    app.namespaces.unselect();
                    app.commands.unselect();
                    app.tags.unselect();

                    app.tabs.current_selected = true;
                }
                None => app.tabs.previous(),
            },
        }
        Ok(None)
    }

    fn move_down(app: &mut App) -> ParserResult {
        if app.namespaces.items.is_empty() {
            return Ok(None);
        }

        match app.commands.state.selected() {
            Some(_) => {
                if !app.commands.items.is_empty() {
                    app.commands.next();
                    app.tags.next();
                }
            }
            None => match app.namespaces.state.selected() {
                Some(_) => {
                    app.namespaces.next();
                    app.set_commands_tags_from_position(app.namespaces.current());
                }
                None => {
                    app.tabs.current_selected = false;

                    app.namespaces.current_selected = true;
                    app.namespaces.state.select(Some(0));
                    app.set_commands_tags_from_position(app.namespaces.current());                }
            },
        }

        Ok(None)
    }

    fn move_up(app: &mut App) -> ParserResult {
        match app.commands.state.selected() {
            Some(_) => {
                if !app.commands.items.is_empty() {
                    app.commands.previous();
                    app.tags.previous();
                }
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
            Some(_) => match app.event_state.get_confirm() {
                Confirm::Display()=> {
                    app.event_state.set_confirm(Confirm::Confirmed);

                    return Ok(Some((
                        app.commands.items[app.commands.current()].clone(),
                        app.tags.items[app.tags.current()].clone(),
                    )));
                }
                Confirm::Hide() => {
                    app.commands.current_selected = false;
                    app.tags.current_selected = false;

                    app.event_state.set_confirm(Confirm::Display);
                }
                _ => {}
            },
            None => match app.namespaces.state.selected() {
                Some(_) => {
                    app.namespaces.current_selected = false;

                    app.commands.current_selected = true;
                    app.tags.current_selected = true;

                    app.commands.state.select(Some(0));
                    app.tags.state.select(Some(0));
                }
                None => {}
            },
        }

        Ok(None)
    }

    fn esc(app: &mut App) -> ParserResult {
        app.event_state.set_mode(Mode::Normal);
        match app.event_state.get_confirm(){
            Confirm::Display => {
                app.commands.current_selected = true;
                app.tags.current_selected = true;

                app.event_state.set_confirm(Confirm::Hide);
            }
            Confirm::Hide => {
                app.commands.current_selected = false;
                app.tags.current_selected = false;

                app.namespaces.current_selected = false;
                app.namespaces.unselect();

                app.tabs.current_selected = true;
            },
            _ => {}
        }

        Ok(None)
    }

    fn change_to_add_mode(app: &mut App) -> ParserResult {
        app.event_state.set_mode(Mode::Add);
        Ok(None)
    }

    fn change_to_delete_mode(app: &mut App) -> ParserResult {
        if app.commands.current_selected || app.namespaces.current_selected {
            app.event_state.set_mode(Mode::Delete);
            app.event_state.set_confirm(Confirm::Display);
        }

        Ok(None)
    }

    fn input_handler(key_code: KeyCode, app: &mut App) -> () {
        match key_code {
            KeyCode::Esc => {
                app.add.add_type = None;
                app.change_mode(Mode::Normal);
            }
            KeyCode::Char(c) => {
                app.add.input.push(c);
                app.cursor_position.as_mut().unwrap().push_inc(c);
            }
            KeyCode::Backspace => {
                app.add.input.pop();
                app.cursor_position.as_mut().unwrap().pop_dec();
            }
            _ => (),
        }
    }

    fn clear_mode(app: &mut App) -> () {
        app.add.input.clear();
        app.add.input_command = None;
        app.add.add_type = None;
        app.change_mode(Mode::Normal);
    }

    fn process_add_namespace(key_code: KeyCode, app: &mut App) -> ParserResult {
        KeyParser::input_handler(key_code, app);

        if key_code == KeyCode::Enter {
            return match app.add.add_type {
                Some(AddType::Namespace) => {
                    if app.add.input.is_empty() {
                        return Ok(None);
                    }

                    let existing_namespace = app.db.get_namespace(&app.add.input);

                    if let Ok(option) = existing_namespace {
                        if let Some(s) = option {
                            let message = format!("Namespace {} already exists", s);

                            app.add.error_message = Some(message);
                            return Ok(None);
                        }
                    }

                    app.db
                        .add_namespace(&app.add.input)
                        .expect("Failed to add namespace");
                    KeyParser::clear_mode(app);

                    let namespaces = app.db.get_namespaces().expect("Failed to get namespaces");
                    app.namespaces = StatefulList::with_items(namespaces);
                    app.cursor_position = None;

                    Ok(None)
                }
                _ => Ok(None),
            };
        }
        Ok(None)
    }

    fn process_add_command(key_code: KeyCode, app: &mut App) -> ParserResult {
        KeyParser::input_handler(key_code, app);

        if key_code == KeyCode::Enter {
            if app.add.input.is_empty() {
                return Ok(None);
            }

            match app.add.input_mode {
                Some(InputMode::Command) => {
                    app.add.input_command = Some(app.add.input.clone());
                    app.add.input.clear();
                    app.add.input_mode = Some(InputMode::Tag);
                }
                Some(InputMode::Tag) => {
                    app.add.input_mode = None;
                    app.db
                        .add_command_and_tag(
                            app.add.input_command.as_ref(),
                            &app.add.input,
                            &app.namespaces.current_item(),
                        )
                        .expect("Failed to add command and tag");

                    KeyParser::clear_mode(app);
                }
                _ => (),
            }

            let (commands, tags) = app
                .db
                .get_commands_and_tags(Some(app.namespaces.current_item().clone()))
                .expect("Failed to get commands and tags");

            app.commands = StatefulList::with_items(commands);
            app.tags = StatefulList::with_items(tags);
            app.cursor_position = None;
        }

        Ok(None)
    }
}
