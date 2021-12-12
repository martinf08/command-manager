use rusqlite::Connection;
use std::error::Error;
use std::path::Path;

const DB_PATH: &str = "~/.cm/command_manager.db";

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
            let db_path = Path::new(DB_PATH);

            let db = db_path
                .to_str()
                .expect("CM_DB path is not a valid UTF-8 string")
                .to_string();

            match db_path.is_file() {
                true => {
                    println!("Using database: {}", db);
                    db
                }
                false => {
                    println!("Creating database: {}", &db);
                    std::fs::create_dir_all(db_path.parent().unwrap())?;
                    create_db_structure(&db)?;
                    dbg!(&db);
                    db
                }
            }
        }
    };

    Ok(db_file)
}

fn create_db_structure(db: &str) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open(db)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS folders (
            id INTEGER PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
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
        );",
        [],
    )?;

    Ok(())
}
