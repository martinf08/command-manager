use rusqlite::{params, Connection};
use std::error::Error;

pub fn db_fixtures(db: &str) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open(db)?;
    let result: u32 = conn.query_row("SELECT count(*) FROM commands", [], |row| row.get(0))?;

    if result > 0 {
        return Ok(());
    }

    // Navigation
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

            INSERT OR IGNORE INTO commands (value, folder_id) VALUES (
                'cd ~/.cargo/bin && $SHELL',
                (SELECT id FROM folders WHERE name = 'navigation')
            );
            INSERT OR IGNORE INTO tags ('name', command_id) VALUES (
                'nav:cargo:bin',
                (
                    SELECT id FROM commands WHERE value = 'cd ~/.cargo/bin && $SHELL'
                    AND folder_id = (SELECT id FROM folders WHERE name = 'navigation')
                )
            );
        ",
    )?;

    // Docker
    conn.execute("INSERT OR IGNORE INTO folders (name) VALUES ('docker')", [])?;

    let docker_id = conn.last_insert_rowid();

    let mut cmd_stmt =
        conn.prepare("INSERT OR IGNORE INTO commands (value, folder_id) VALUES (?, ?)")?;

    let mut tag_stmt =
        conn.prepare("INSERT OR IGNORE INTO tags (name, command_id) VALUES (?, ?)")?;

    cmd_stmt.execute(params![
        "docker run --rm --name alpine -it alpine",
        docker_id
    ])?;

    tag_stmt.execute(params!["docker:run:alpine", conn.last_insert_rowid()])?;

    cmd_stmt.execute(params![
        r#"echo "Removing containers :" && if [ -n "$(docker container ls -aq)" ]; then docker container stop $(docker container ls -aq); docker container rm $(docker container ls -aq); fi; echo "Removing images :" && if [ -n "$(docker images -aq)" ]; then docker rmi -f $(docker images -aq); fi; echo "Removing volumes :" && if [ -n "$(docker volume ls -q)" ]; then docker volume rm $(docker volume ls -q); fi; echo "Removing networks :" && if [ -n "$(docker network ls | awk '{print $1" "$2}' | grep -v 'ID\|bridge\|host\|none' | awk '{print $1}')" ]; then docker network rm $(docker network ls | awk '{print $1" "$2}' | grep -v 'ID\|bridge\|host\|none' | awk '{print $1}'); fi"#,
        docker_id]
    )?;

    tag_stmt.execute(params!["docker:purge", conn.last_insert_rowid()])?;

    Ok(())
}
