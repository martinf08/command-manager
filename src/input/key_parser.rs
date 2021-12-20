use crate::app::State;
use crate::App;
use crossterm::event::{KeyCode, KeyEvent};
use std::error::Error;

pub struct KeyParser;

pub type KeyParserResult = Result<Option<(String, String)>, Box<dyn Error>>;

impl KeyParser {
    pub fn parse_key(key_event: KeyEvent, app: &mut App) -> KeyParserResult {
        KeyParser::process_key_code(key_event.code, app)
    }

    fn process_key_code(key_code: KeyCode, app: &mut App) -> KeyParserResult {
        if key_code == KeyCode::Char('q') {
            KeyParser::quit(app);
            return Ok(None);
        }

        match app.tabs.index {
            0 => KeyParser::process_tab_0(key_code, app),
            // 1 => KeyParser::process_tab_1(key_code, app),
            // 2 => KeyParser::process_tab_2(key_code, app),
            _ => Ok(None),
        }
    }

    fn quit(app: &mut App) -> KeyParserResult {
        app.quit = true;
        Ok(None)
    }

    fn process_tab_0(key_code: KeyCode, app: &mut App) -> KeyParserResult {
        match key_code {
            KeyCode::Right => KeyParser::move_right(app),
            KeyCode::Left => KeyParser::move_left(app),
            KeyCode::Down => KeyParser::move_down(app),
            KeyCode::Up => KeyParser::move_up(app),
            KeyCode::Enter => KeyParser::enter(app),
            KeyCode::Esc => KeyParser::esc(app),
            _ => Ok(None),
        }
    }

    fn move_right(app: &mut App) -> KeyParserResult {
        match app.folders.state.selected() {
            Some(_) => {
                app.switch_selected_widgets_off();
                app.switch_selected_commands_tags_on()
            }
            None => app.tabs.next(),
        }

        Ok(None)
    }

    fn move_left(app: &mut App) -> KeyParserResult {
        match app.commands.state.selected() {
            Some(_) => {
                app.switch_selected_commands_tags_off();
                app.set_current_selected_folder(true)
            }
            None => match app.folders.state.selected() {
                Some(_) => {
                    app.switch_selected_folders_off();
                    app.switch_selected_commands_tags_off();
                    app.set_current_selected_tab(true);
                }
                None => app.tabs.previous(),
            },
        }
        Ok(None)
    }

    fn move_down(app: &mut App) -> KeyParserResult {
        match app.commands.state.selected() {
            Some(_) => {
                app.commands.next();
                app.tags.next();
            }
            None => match app.folders.state.selected() {
                Some(_) => {
                    app.folders.next();
                    app.set_commands_tags_from_position(app.folders.current());
                }
                None => {
                    app.set_current_selected_tab(false);
                    app.switch_selected_folders_on();
                }
            },
        }

        Ok(None)
    }

    fn move_up(app: &mut App) -> KeyParserResult {
        match app.commands.state.selected() {
            Some(_) => {
                app.commands.previous();
                app.tags.previous();
            }
            None => {
                app.folders.previous();
                app.set_commands_tags_from_position(app.folders.current());
            }
        };

        Ok(None)
    }

    fn enter(app: &mut App) -> KeyParserResult {
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
            None => match app.folders.state.selected() {
                Some(_) => {
                    app.set_current_selected_folder(false);
                    app.switch_selected_commands_tags_on();
                }
                None => {}
            },
        }

        Ok(None)
    }

    fn esc(app: &mut App) -> KeyParserResult {
        match app.show_command_confirmation {
            true => {
                app.set_current_selected_commands_tags(true);
                app.set_show_command_confirmation(false);
            }
            false => {
                app.switch_selected_commands_tags_off();
                app.switch_selected_folders_off();
                app.set_current_selected_tab(true);
            }
        }

        Ok(None)
    }
}