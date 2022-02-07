#![allow(unused)]
use crate::fixtures;
use rusqlite::Connection;
use std::error::Error;
use std::path::Path;

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
            let db_namespace = home.join(".cm");
            std::fs::create_dir_all(&db_namespace)?;
            let db_path = db_namespace.join("command_manager.db");
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
    CREATE TABLE IF NOT EXISTS namespaces (
        id INTEGER PRIMARY KEY,
        name VARCHAR(255) UNIQUE NOT NULL
    );
    CREATE TABLE IF NOT EXISTS commands (
        id INTEGER PRIMARY KEY,
        value TEXT NOT NULL,
        namespace_id INTEGER NOT NULL,
        FOREIGN KEY (namespace_id) REFERENCES namespaces(id)
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

pub fn add_namespace(s: &String) -> Result<(), Box<dyn Error>> {
    let db = get_db()?;
    let conn = Connection::open(db)?;

    let mut stmt = conn.prepare("INSERT INTO namespaces (name) VALUES (?)")?;
    stmt.execute([s]);

    Ok(())
}

pub fn get_namespaces() -> Result<Vec<String>, Box<dyn Error>> {
    let db = get_db()?;
    let conn = Connection::open(db)?;

    let mut namespaces: Vec<String> = Vec::new();

    let mut stmt = conn.prepare("SELECT name FROM namespaces")?;
    for row in stmt.query_map([], |row| row.get(0))? {
        let namespace = row?;
        namespaces.push(namespace);
    }

    Ok(namespaces)
}

pub fn get_namespace(s: &String) -> Result<Option<String>, Box<dyn Error>> {
    let db = get_db()?;
    let conn = Connection::open(db)?;

    let mut stmt = conn.prepare("SELECT name FROM namespaces WHERE name = ?")?;
    let mut rows = stmt.query([s])?;

    let mut namespace = None;
    if let Ok(row) = rows.next() {
        namespace = if row.is_none() {
            None
        } else {
            Some(row.ok_or("Unable to get row")?.get(0)?)
        };
    }

    if namespace.is_none() {
        return Ok(None);
    }

    Ok(namespace)
}

pub fn get_commands_and_tags(
    namespace: Option<String>,
) -> Result<(Vec<String>, Vec<String>), Box<dyn Error>> {
    let db = get_db()?;
    let conn = Connection::open(db)?;

    let mut commands: Vec<String> = Vec::new();
    let mut tags: Vec<String> = Vec::new();

    if namespace.is_some() {
        let mut stmt = conn.prepare(
            r"
            SELECT commands.value, tags.name FROM commands
            JOIN tags ON tags.command_id = commands.id
            WHERE namespace_id = (SELECT id FROM namespaces WHERE name = :namespace);",
        )?;

        stmt.query_map([namespace.unwrap()], |row| {
            let command = row.get(0)?;
            let tag = row.get(1)?;
            Ok((command, tag))
        })?
        .for_each(|row| {
            let (command, tag) = row.expect("Unable to get row");
            commands.push(command);
            tags.push(tag);
        });
    } else {
        let mut stmt = conn.prepare(
            r"
            SELECT commands.value, tags.name FROM commands
            JOIN tags ON tags.command_id = commands.id
            WHERE namespace_id = (SELECT id FROM namespaces LIMIT 1);",
        )?;
        stmt.query_map([], |row| {
            let command = row.get(0)?;
            let tag = row.get(1)?;
            Ok((command, tag))
        })?
        .for_each(|row| {
            let (command, tag) = row.expect("Unable to get row");
            commands.push(command);
            tags.push(tag);
        });
    };

    if commands.len() != tags.len() {
        panic!("Commands and tags are not the same length");
    }

    Ok((commands, tags))
}

pub fn add_command(command: &String, namespace: &String) -> Result<(), Box<dyn Error>> {
    let db = get_db()?;
    let conn = Connection::open(db)?;

    let mut stmt = conn.prepare(
        r"
        INSERT INTO commands (value, namespace_id)
        VALUES (:command, (SELECT id FROM namespaces WHERE name = :namespace));",
    )?;

    stmt.execute([command, namespace])?;
    Ok(())
}
