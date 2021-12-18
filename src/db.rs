use rusqlite::{Connection, NO_PARAMS};
use std::error::Error;
use std::path::Path;

mod fixtures;

pub fn init_db(init_fixtures: bool) -> Result<(), Box<dyn Error>> {
    let db = get_db()?;
    create_db_structure(&db)?;
    if init_fixtures {
        fixtures::db_fixtures(&db)?;
    }

    Ok(())
}

pub fn get_db() -> Result<String, Box<dyn Error>> {
    let db_file = std::env::var("CM_DB");

    let db_file = match db_file {
        Ok(f) => {
            let db_path = Path::new(&f);
            if !db_path.is_file() {
                panic!("CM_DB env is not a file");
            }
            f
        }
        Err(_) => {
            let home = dirs::home_dir().expect("Could not find home directory");
            let db_folder = home.join(".cm");
            std::fs::create_dir_all(&db_folder)?;
            let db_path = db_folder.join("command_manager.db");
            let db = db_path.to_str().expect("Unable to get db path");

            db.to_string()
        }
    };

    Ok(db_file)
}

fn create_db_structure(db: &str) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open(db)?;

    conn.execute_batch(
        r"
    CREATE TABLE IF NOT EXISTS folders (
        id INTEGER PRIMARY KEY,
        name VARCHAR(255) UNIQUE NOT NULL
    );
    CREATE TABLE IF NOT EXISTS commands (
        id INTEGER PRIMARY KEY,
        value TEXT NOT NULL,
        folder_id INTEGER NOT NULL,
        FOREIGN KEY (folder_id) REFERENCES folders(id)
    );
    CREATE TABLE IF NOT EXISTS tags (
        id INTEGER PRIMARY KEY,
        name VARCHAR(255) UNIQUE NOT NULL,
        command_id INTEGER NOT NULL,
        FOREIGN KEY (command_id) REFERENCES commands(id)
    );
    ",
    )?;

    Ok(())
}

pub fn get_folders() -> Result<Vec<String>, Box<dyn Error>> {
    let db = get_db()?;
    let conn = Connection::open(db)?;

    let mut folders: Vec<String> = Vec::new();

    let mut stmt = conn.prepare("SELECT name FROM folders")?;
    for row in stmt.query_map([], |row| row.get(0))? {
        let folder = row?;
        folders.push(folder);
    }

    Ok(folders)
}

pub fn get_commands(folder: Option<String>) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let db = get_db()?;
    let conn = Connection::open(db)?;

    let mut commands: Vec<(String, String)> = Vec::new();

    if folder.is_some() {
        let mut stmt = conn.prepare(
            r"
            SELECT commands.value, tags.name FROM commands
            JOIN tags ON tags.command_id = commands.id
            WHERE folder_id = (SELECT id FROM folders WHERE name = :folder);",
        )?;

        stmt.query_map([folder.unwrap()], |row| {
            let command = row.get(0)?;
            let tag = row.get(1)?;
            Ok((command, tag))
        })?
        .for_each(|row| {
            let (command, tag) = row.expect("Unable to get row");
            commands.push((command, tag));
        });
    } else {
        let mut stmt = conn.prepare(
            r"
            SELECT commands.value, tags.name FROM commands
            JOIN tags ON tags.command_id = commands.id
            WHERE folder_id = (SELECT id FROM folders LIMIT 1);",
        )?;
        stmt.query_map([], |row| {
            let command = row.get(0)?;
            let tag = row.get(1)?;
            Ok((command, tag))
        })?
        .for_each(|row| {
            let (command, tag) = row.expect("Unable to get row");
            commands.push((command, tag));
        });
    };
    Ok(commands)
}
