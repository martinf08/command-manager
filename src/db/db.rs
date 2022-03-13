#![allow(unused)]

use crate::db::fixtures;
use rusqlite::Connection;
use std::error::Error;
use std::io::ErrorKind;
use std::path::Path;

pub struct Db {
    conn: Connection,
}

impl Db {
    pub fn new() -> Result<Db, Box<dyn Error>> {
        let path = Db::get_db_path()?;
        let conn = Connection::open(path)?;

        Ok(Db { conn })
    }

    pub fn get_db_path() -> Result<String, Box<dyn Error>> {
        let db_file = std::env::var("CM_DB");

        let db_file = match db_file {
            Ok(f) => {
                let db_path = Path::new(&f);
                if !db_path.is_file() {
                    return Err(Box::new(std::io::Error::new(
                        ErrorKind::NotFound,
                        "CM_DB env var is not a file",
                    )));
                }
                Ok(f)
            }
            Err(_) => {
                let home = dirs::home_dir().ok_or("No home directory found")?;
                let db_namespace = home.join(".cm");
                std::fs::create_dir_all(&db_namespace)?;
                let db_path = db_namespace.join("command_manager.db");
                let db = db_path
                    .to_str()
                    .ok_or("Could not convert db path to string")?;

                Ok(db.to_string())
            }
        };

        db_file
    }

    pub fn init_db(&self) -> Result<(), Box<dyn Error>> {
        self.create_db_structure()?;
        fixtures::db_fixtures(&self.conn)?;

        Ok(())
    }

    pub fn create_db_structure(&self) -> Result<(), Box<dyn Error>> {
        self.conn.execute_batch(
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
        ON DELETE CASCADE
    );
    CREATE TABLE IF NOT EXISTS tags (
        id INTEGER PRIMARY KEY,
        name VARCHAR(255) UNIQUE NOT NULL,
        command_id INTEGER NOT NULL,
        FOREIGN KEY (command_id) REFERENCES commands(id)
        ON DELETE CASCADE
    );
    ",
        )?;

        Ok(())
    }

    pub fn add_namespace(&self, s: &String) -> Result<(), Box<dyn Error>> {
        let mut stmt = self
            .conn
            .prepare("INSERT INTO namespaces (name) VALUES (?)")?;
        stmt.execute([s]);

        Ok(())
    }

    pub fn get_namespaces(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let mut namespaces: Vec<String> = Vec::new();

        let mut stmt = self.conn.prepare("SELECT name FROM namespaces")?;
        for row in stmt.query_map([], |row| row.get(0))? {
            let namespace = row?;
            namespaces.push(namespace);
        }

        Ok(namespaces)
    }

    pub fn get_namespace(&self, s: &String) -> Result<Option<String>, Box<dyn Error>> {
        let mut stmt = self
            .conn
            .prepare("SELECT name FROM namespaces WHERE name = ?")?;
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
        &self,
        namespace: Option<String>,
    ) -> Result<(Vec<String>, Vec<String>), Box<dyn Error>> {
        let mut commands: Vec<String> = Vec::new();
        let mut tags: Vec<String> = Vec::new();

        if namespace.is_some() {
            let mut stmt = self.conn.prepare(
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
            let mut stmt = self.conn.prepare(
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

    pub fn add_command_and_tag(
        &self,
        command: Option<&String>,
        tag: Option<&String>,
        namespace: &String,
    ) -> Result<(), Box<dyn Error>> {
        let mut stmt = self.conn.prepare(
            r"
        INSERT INTO commands (value, namespace_id)
        VALUES (:command, (SELECT id FROM namespaces WHERE name = :namespace));",
        )?;

        stmt.execute([command, Some(namespace)])?;

        let mut stmt = self.conn.prepare(
            r"
        INSERT INTO tags (name, command_id)
        VALUES (:tag, (SELECT id FROM commands WHERE value = :command));",
        )?;

        stmt.execute([tag, command])?;
        Ok(())
    }

    pub fn delete_command(
        &self,
        command: &String,
        namespace: &String,
    ) -> Result<(), Box<dyn Error>> {
        self.conn.execute(r"SET FOREIGN_KEY_CHECKS=0", []);

        let mut stmt = self.conn.prepare(
            r"
        DELETE FROM commands
        WHERE value = :command AND namespace_id = (SELECT id FROM namespaces WHERE name = :namespace);",
        )?;

        stmt.execute([command, namespace])?;

        let mut stmt = self.conn.prepare(
            r"
        DELETE FROM tags
        WHERE command_id = (SELECT id FROM commands WHERE value = :command);",
        )?;

        stmt.execute([command])?;

        self.conn.execute(r"SET FOREIGN_KEY_CHECKS=1", []);

        Ok(())
    }

    pub fn delete_namespace(&self, namespace: &String) -> Result<(), Box<dyn Error>> {
        let mut stmt = self.conn.prepare(
            r"
        DELETE FROM commands
        WHERE namespace_id = (SELECT id FROM namespaces WHERE name = :namespace);",
        )?;

        stmt.execute([namespace])?;

        let mut stmt = self.conn.prepare(
            r"
        DELETE FROM tags
        WHERE command_id IN (SELECT id FROM commands WHERE namespace_id = (SELECT id FROM namespaces WHERE name = :namespace));",
        )?;

        stmt.execute([namespace])?;

        let mut stmt = self.conn.prepare(
            r"
        DELETE FROM namespaces
        WHERE name = :namespace;",
        )?;

        stmt.execute([namespace])?;
        Ok(())
    }
}
