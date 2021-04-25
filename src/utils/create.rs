use anyhow::Result as AnyResult;
use rusqlite::Connection;

pub fn create_db(conn: &Connection) -> AnyResult<()> {
  conn.execute(
    "CREATE TABLE IF NOT EXISTS event (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    content     TEXT NOT NULL
    )",
    [],
  )?;
  Ok(())
}

//conn.execute(
//"CREATE TABLE event (
//id        INTEGER PRIMARY KEY,
//content   TEXT NOT NULL
//)",
//[],
//)?;
//println!("table created");
