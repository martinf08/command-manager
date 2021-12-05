use crossterm::event;
use crossterm::event::{Event, KeyCode};
use linefeed::{Interface, ReadResult};
use regex::Regex;
use std::collections::HashMap;
use std::io;
use std::process::Command as StdCommand;
use std::time::Duration;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, ListItem, ListState};
use tui::{Frame, Terminal};

struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> Self {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
}

pub struct App {
    items: StatefulList<String>,
}

impl App {
    pub fn init_from_history_file(history_file: String) -> Self {
        let output = StdCommand::new("sh")
            .arg("-c")
            .arg(format!("cat {}", history_file))
            .output()
            .unwrap();

        let raw_history = String::from_utf8_lossy(&output.stdout);

        let mut ranking = HashMap::new();
        raw_history.split('\n').for_each(|line| {
            let cmd = line.split(';').last().unwrap_or_default();
            ranking.entry(cmd).and_modify(|e| *e += 1).or_insert(1);
        });

        let mut sort_ranking = ranking.into_iter().collect::<Vec<(&str, usize)>>();

        sort_ranking.sort_by(|a, b| b.1.cmp(&a.1));

        let items: Vec<String> = sort_ranking
            .into_iter()
            .map(|(cmd, count)| {
                let mut cmd = String::from(cmd);
                cmd.push_str(format!(" ({})", count).as_str());
                cmd
            })
            .collect();

        App {
            items: StatefulList::with_items(items),
        }
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<Option<String>> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(None),
                    KeyCode::Left => app.items.unselect(),
                    KeyCode::Down => app.items.next(),
                    KeyCode::Up => app.items.previous(),
                    KeyCode::Enter => {
                        return Ok(app
                            .items
                            .state
                            .selected()
                            .map(|i| app.items.items[i].clone()))
                    }
                    _ => {}
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(5),
                Constraint::Percentage(40),
                Constraint::Percentage(40),
                Constraint::Percentage(5),
            ]
            .as_ref(),
        )
        .split(f.size());

    let block = Block::default().title("History").borders(Borders::ALL);
    f.render_widget(block, chunks[0]);

    let items = app
        .items
        .items
        .iter()
        .filter(|item| !item.trim().is_empty())
        .map(|item| ListItem::new(item.as_str()).style(Style::default().fg(Color::Yellow)))
        .collect::<Vec<ListItem>>();

    let list = List::new(items)
        .block(Block::default().title("List").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>");

    f.render_stateful_widget(list, chunks[1], &mut app.items.state);
}

pub fn write_command_in_terminal(result: String) -> io::Result<()> {
    let re = Regex::new(r"^(.+)(\s\(\d+\))$").unwrap();
    let cap = re.captures(&result).expect("Invalid command");
    let value = cap.get(1).unwrap().as_str();

    let reader = Interface::new("my-application").unwrap();
    reader.set_prompt("Command-manager>").unwrap();
    reader.set_buffer(value).unwrap();

    while let ReadResult::Input(input) = reader.read_line().unwrap() {
        let output = StdCommand::new("sh").arg("-c").arg(&input).output()?;

        let mut writer = reader.lock_writer_append().unwrap();
        let lossy_str = String::from_utf8_lossy(&output.stdout);
        writer.write_str(&*lossy_str).unwrap();
        drop(writer);
        break;
    }

    Ok(())
}
