use rusqlite::{Batch, Connection};
use std::error::Error;
use std::path::Path;

pub fn init_db() -> Result<String, Box<dyn Error>> {
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

            create_db_structure(db)?;

            db.to_string()
        }
    };

    Ok(db_file)
}

fn create_db_structure(db: &str) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open(db)?;

    let sql = r"
    CREATE TABLE IF NOT EXISTS folders (
        id INTEGER PRIMARY KEY,
        name VARCHAR(255) NOT NULL
    );
    CREATE TABLE IF NOT EXISTS commands (
        id INTEGER PRIMARY KEY,
        value TEXT NOT NULL,
        folder_id INTEGER NOT NULL,
        FOREIGN KEY (folder_id) REFERENCES folders(id)
    );
    CREATE TABLE IF NOT EXISTS tags (
        id INTEGER PRIMARY KEY,
        name VARCHAR(255) NOT NULL,
        command_id INTEGER NOT NULL,
        FOREIGN KEY (command_id) REFERENCES commands(id)
    );
    ";

    let mut batch = Batch::new(&conn, sql);
    while let Some(mut stmt) = batch.next()? {
        stmt.execute([])?;
    }

    Ok(())
}
