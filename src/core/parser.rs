use crate::app::app::App;
use crate::app::event_state::{Confirm, EventState, EventType, Mode, SubMode, Tab};
use crate::app::state::State;
use crossterm::event::{KeyCode, KeyEvent};
use std::error::Error;

pub struct KeyParser;

pub type ParserResult = Result<Option<(String, String)>, Box<dyn Error>>;

impl KeyParser {
    pub fn parse_event(key_event: KeyEvent, app: &mut App) -> ParserResult {
        KeyParser::process_key_code(key_event.code, app)
    }

    fn quit(app: &mut App) -> ParserResult {
        app.quit = true;
        Ok(None)
    }

    fn process_key_code(key_code: KeyCode, app: &mut App) -> ParserResult {
        if key_code == KeyCode::Char('q')
            && (app.event_state.get_mode() == &Mode::Normal
                || app.event_state.get_mode() == &Mode::Delete)
        {
            KeyParser::quit(app)?;
            return Ok(None);
        }

        if key_code == KeyCode::Esc {
            app.event_state = EventState::default();
            return Ok(None);
        }

        match app.event_state.get_tab() {
            Tab::Tab1 => KeyParser::process_tab_1(key_code, app),
            Tab::Tab2 => KeyParser::process_tab_2(key_code, app),
            Tab::Tab3 => KeyParser::process_tab_3(key_code, app),
            _ => Ok(None),
        }
    }

    fn process_tab_1(key_code: KeyCode, app: &mut App) -> ParserResult {
        match app.event_state.get_mode() {
            Mode::Normal => KeyParser::process_normal_mode(key_code, app),
            Mode::Add => KeyParser::process_add_mode(key_code, app),
            Mode::Delete => KeyParser::process_delete_mode(key_code, app),
            _ => Ok(None),
        }
    }

    fn process_tab_2(key_code: KeyCode, app: &mut App) -> ParserResult {
        let mut tabs = app.tabs.as_ref().borrow_mut();
        match key_code {
            KeyCode::Right | KeyCode::Char('l') => {
                tabs.next();
                Ok(None)
            }
            KeyCode::Left | KeyCode::Char('h') => {
                tabs.previous();
                Ok(None)
            }
            _ => Ok(None),
        }
    }

    fn process_tab_3(key_code: KeyCode, app: &mut App) -> ParserResult {
        let mut tabs = app.tabs.as_ref().borrow_mut();
        match key_code {
            KeyCode::Right | KeyCode::Char('l') => {
                tabs.next();
                Ok(None)
            }
            KeyCode::Left | KeyCode::Char('h') => {
                tabs.previous();
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
            KeyCode::Enter | KeyCode::Char(' ') => KeyParser::enter(app),
            KeyCode::Char('n') => KeyParser::change_to_add_namespace_mode(app),
            KeyCode::Char('a') => KeyParser::change_to_add_command_mode(app),
            KeyCode::Char('d') => KeyParser::change_to_delete_mode(app),
            _ => Ok(None),
        }
    }

    fn process_add_mode(key_code: KeyCode, app: &mut App) -> ParserResult {
        match app.event_state.get_sub_mode() {
            SubMode::Namespace => match app.event_state.get_confirm() {
                Confirm::Hide => {
                    KeyParser::input_handler(
                        key_code,
                        app,
                        app.config.name_config.namespace.to_string(),
                    );
                    Ok(None)
                }
                Confirm::Display => KeyParser::process_add_namespace_mode_confirm(key_code, app),
                _ => Ok(None),
            },
            SubMode::Command => match app.event_state.get_event_type() {
                EventType::Namespace => {
                    if app.event_state.get_confirm() == &Confirm::Display {
                        app.event_state.set_event_type(EventType::Tag);
                        app.event_state.set_confirm(Confirm::Hide);

                        return Ok(None)
                    }

                    KeyParser::input_handler(
                        key_code,
                        app,
                        app.config.name_config.command.to_string(),
                    );

                    Ok(None)
                },
                EventType::Tag => {
                    if app.event_state.get_confirm() == &Confirm::Display {
                        KeyParser::process_add_command_mode_confirm(key_code, app);

                        return Ok(None)
                    }

                    KeyParser::input_handler(
                        key_code,
                        app,
                        app.config.name_config.tag.to_string(),
                    );

                    Ok(None)
                },
                _ => Ok(None),
            },
            _ => Ok(None),
        }
    }

    fn process_add_command_mode_confirm(key_code: KeyCode, app: &mut App) -> ParserResult {
        match key_code {
            KeyCode::Enter | KeyCode::Char(' ') => {

                //Todo

                Ok(None)
            },
            _ => Ok(None),
        }
    }

    fn process_add_namespace_mode_confirm(key_code: KeyCode, app: &mut App) -> ParserResult {
        match key_code {
            KeyCode::Enter | KeyCode::Char(' ') => {
                if app.inputs.is_empty() {
                    return Ok(None);
                }

                let namespace = app
                    .inputs
                    .remove("namespace")
                    .expect("namespace input is empty")
                    .iter()
                    .collect::<String>();

                let existing_namespace = app.db.get_namespace(&namespace)?;

                if existing_namespace.is_some() {
                    // let message = format!("Namespace {} already exists", s);
                    //
                    // app.add.error_message = Some(message);
                    //Todo message
                    return Ok(None);
                }

                app.db.add_namespace(&namespace)?;

                let namespaces = app.db.get_namespaces()?;

                let mut app_namespace = app.namespaces.as_ref().borrow_mut();
                app_namespace.items = namespaces;
                app_namespace.state.select(Some(0));

                app.cursor_position = None;
                app.event_state.set_confirm(Confirm::Confirmed);

                Ok(None)
            }
            _ => Ok(None),
        }
    }

    fn process_delete_mode(key_code: KeyCode, app: &mut App) -> ParserResult {
        let mut app_commands = app.commands.as_ref().borrow_mut();
        let mut app_namespaces = app.namespaces.as_ref().borrow_mut();
        let mut app_tags = app.tags.as_ref().borrow_mut();

        if app.event_state.get_confirm() != &Confirm::Display {
            app.event_state.set_confirm(Confirm::Confirmed);
            return Ok(None);
        }

        match key_code {
            KeyCode::Enter | KeyCode::Char(' ') => {
                if app_commands.is_selected {
                    app.db.delete_command(
                        app_commands.current_item(),
                        app_namespaces.current_item(),
                    )?;

                    let (commands, tags) = app
                        .db
                        .get_commands_and_tags(Some(app_namespaces.current_item().clone()))?;

                    app_commands.items = commands;
                    app_commands.state.select(Some(0));

                    app_tags.items = tags;
                    app_tags.state.select(Some(0));
                } else if app_namespaces.is_selected {
                    app.db.delete_namespace(app_namespaces.current_item())?;

                    let namespaces = app.db.get_namespaces()?;
                    app_namespaces.items = namespaces;
                    app_namespaces.state.select(Some(0));

                    let (commands, tags) = app
                        .db
                        .get_commands_and_tags(Some(app_namespaces.current_item().clone()))?;

                    app_commands.items = commands;
                    app_commands.state.select(Some(0));

                    app_tags.items = tags;
                    app_tags.state.select(Some(0));
                } else {
                    return Ok(None);
                }

                app.event_state.set_confirm(Confirm::Confirmed);
                app.event_state.set_mode(Mode::Normal);

                return Ok(None);
            }
            _ => {}
        }

        Ok(None)
    }

    fn move_right(app: &mut App) -> ParserResult {
        let mut tabs = app.tabs.as_ref().borrow_mut();
        let mut namespaces = app.namespaces.as_ref().borrow_mut();
        let mut commands = app.commands.as_ref().borrow_mut();
        let mut tags = app.tags.as_ref().borrow_mut();

        match namespaces.state.selected() {
            Some(_) => {
                namespaces.is_selected = false;

                commands.is_selected = true;
                tags.is_selected = true;

                commands.state.select(Some(0));
                tags.state.select(Some(0));
            }
            None => tabs.next(),
        }

        Ok(None)
    }

    fn move_left(app: &mut App) -> ParserResult {
        let mut tabs = app.tabs.as_ref().borrow_mut();
        let mut namespaces = app.namespaces.as_ref().borrow_mut();
        let mut commands = app.commands.as_ref().borrow_mut();
        let mut tags = app.tags.as_ref().borrow_mut();

        match commands.state.selected() {
            Some(_) => {
                namespaces.is_selected = true;

                commands.is_selected = false;
                tags.is_selected = false;

                commands.unselect();
                tags.unselect();
            }
            None => match namespaces.state.selected() {
                Some(_) => {
                    namespaces.is_selected = false;
                    commands.is_selected = false;
                    tags.is_selected = false;

                    namespaces.unselect();
                    commands.unselect();
                    tags.unselect();

                    tabs.is_selected = true;
                }
                None => tabs.previous(),
            },
        }
        Ok(None)
    }

    fn move_down(app: &mut App) -> ParserResult {
        let mut tabs = app.tabs.as_ref().borrow_mut();
        let mut namespaces = app.namespaces.as_ref().borrow_mut();
        let mut commands = app.commands.as_ref().borrow_mut();
        let mut tags = app.tags.as_ref().borrow_mut();

        if namespaces.items.is_empty() {
            return Ok(None);
        }

        match commands.state.selected() {
            Some(_) => {
                if !commands.items.is_empty() {
                    commands.next();
                    tags.next();
                }
            }
            None => match namespaces.state.selected() {
                Some(_) => {
                    namespaces.next();

                    let index = namespaces.current();
                    let namespace = namespaces.items[index].clone();

                    let (new_commands, new_tags) = app.db.get_commands_and_tags(Some(namespace))?;
                    commands.items = new_commands;
                    tags.items = new_tags;
                }
                None => {
                    tabs.is_selected = false;

                    namespaces.is_selected = true;
                    namespaces.state.select(Some(0));

                    let index = namespaces.current();
                    let namespace = namespaces.items[index].clone();

                    let (new_commands, new_tags) = app.db.get_commands_and_tags(Some(namespace))?;
                    commands.items = new_commands;
                    tags.items = new_tags;
                }
            },
        }

        Ok(None)
    }

    fn move_up(app: &mut App) -> ParserResult {
        let mut namespaces = app.namespaces.as_ref().borrow_mut();
        let mut commands = app.commands.as_ref().borrow_mut();
        let mut tags = app.tags.as_ref().borrow_mut();

        match commands.state.selected() {
            Some(_) => {
                if !commands.items.is_empty() {
                    commands.previous();
                    tags.previous();
                }
            }
            None => {
                namespaces.previous();

                let index = namespaces.current();
                let namespace = namespaces.items[index].clone();

                let (new_commands, new_tags) = app.db.get_commands_and_tags(Some(namespace))?;
                commands.items = new_commands;
                tags.items = new_tags;
            }
        };

        Ok(None)
    }

    fn enter(app: &mut App) -> ParserResult {
        let mut namespaces = app.namespaces.as_ref().borrow_mut();
        let mut commands = app.commands.as_ref().borrow_mut();
        let mut tags = app.tags.as_ref().borrow_mut();

        match commands.state.selected() {
            Some(_) => match app.event_state.get_confirm() {
                Confirm::Display => {
                    if commands.items.is_empty() {
                        app.event_state.set_confirm(Confirm::Confirmed);
                        return Ok(None);
                    }

                    app.event_state.set_confirm(Confirm::Confirmed);

                    return Ok(Some((
                        commands.items[commands.current()].clone(),
                        tags.items[tags.current()].clone(),
                    )));
                }
                Confirm::Hide => {
                    commands.is_selected = false;
                    tags.is_selected = false;

                    app.event_state.set_confirm(Confirm::Display);
                }
                _ => {}
            },
            None => match namespaces.state.selected() {
                Some(_) => {
                    namespaces.is_selected = false;

                    commands.is_selected = true;
                    tags.is_selected = true;

                    commands.state.select(Some(0));
                    tags.state.select(Some(0));
                }
                None => {}
            },
        }

        Ok(None)
    }

    fn change_to_add_namespace_mode(app: &mut App) -> ParserResult {
        app.event_state = EventState::default();
        app.event_state.set_mode(Mode::Add);
        app.event_state.set_sub_mode(SubMode::Namespace);
        app.event_state.set_event_type(EventType::Namespace);

        Ok(None)
    }

    fn change_to_add_command_mode(app: &mut App) -> ParserResult {
        app.event_state = EventState::default();
        app.event_state.set_mode(Mode::Add);
        app.event_state.set_sub_mode(SubMode::Command);
        app.event_state.set_event_type(EventType::Command);

        Ok(None)
    }

    fn change_to_delete_mode(app: &mut App) -> ParserResult {
        let commands = app.commands.as_ref().borrow();
        let namespaces = app.namespaces.as_ref().borrow();

        if commands.is_selected || namespaces.is_selected {
            app.event_state.set_mode(Mode::Delete);
            app.event_state.set_confirm(Confirm::Display);
        }

        Ok(None)
    }

    fn input_handler(key_code: KeyCode, app: &mut App, k: String) -> () {
        match key_code {
            KeyCode::Enter => {
                app.event_state.set_confirm(Confirm::Display);
            }
            KeyCode::Char(c) => {
                app.inputs.entry(k).or_insert_with(Vec::new).push(c);
                app.cursor_position.as_mut().unwrap().push_inc(c);
            }
            KeyCode::Backspace => {
                app.inputs.entry(k).or_insert_with(Vec::new).pop();
                app.cursor_position.as_mut().unwrap().pop_dec();
            }
            _ => (),
        }
    }

    // fn process_add_command(key_code: KeyCode, app: &mut App) -> ParserResult {
    //     KeyParser::input_handler(key_code, app);
    //
    //     if key_code == KeyCode::Enter {
    //         if app.add.input.is_empty() {
    //             return Ok(None);
    //         }
    //
    //         match app.add.input_mode {
    //             Some(InputMode::Command) => {
    //                 app.add.input_command = Some(app.add.input.clone());
    //                 app.add.input.clear();
    //                 app.add.input_mode = Some(InputMode::Tag);
    //             }
    //             Some(InputMode::Tag) => {
    //                 app.add.input_mode = None;
    //                 app.db
    //                     .add_command_and_tag(
    //                         app.add.input_command.as_ref(),
    //                         &app.add.input,
    //                         &app.namespaces.current_item(),
    //                     )
    //                     .expect("Failed to add command and tag");
    //
    //                 KeyParser::clear_mode(app);
    //             }
    //             _ => (),
    //         }
    //
    //         let (commands, tags) = app
    //             .db
    //             .get_commands_and_tags(Some(app.namespaces.current_item().clone()))
    //             .expect("Failed to get commands and tags");
    //
    //         app.commands = StatefulList::with_items(commands);
    //         app.tags = StatefulList::with_items(tags);
    //         app.cursor_position = None;
    //     }
    //
    //     Ok(None)
    // }
}
