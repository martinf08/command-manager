use rusqlite::Connection;
use std::error::Error;

pub fn db_fixtures(db: &str) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open(db)?;
    let result: u32 = conn.query_row("SELECT count(*) FROM commands", [], |row| row.get(0))?;

    if result > 0 {
        return Ok(());
    }

    conn.execute_batch(
        "
            INSERT OR IGNORE INTO folders (name) VALUES ('navigation');
            INSERT OR IGNORE INTO commands (value, folder_id) VALUES (
                'cd ~/ && $SHELL',
                (SELECT id FROM folders WHERE name = 'navigation')
            );
            INSERT OR IGNORE INTO tags ('name', command_id) VALUES (
                'nav:home',
                (
                    SELECT id FROM commands WHERE value = 'cd ~/ && $SHELL'
                    AND folder_id = (SELECT id FROM folders WHERE name = 'navigation')
                )
            );
        ",
    )?;

    Ok(())
}
