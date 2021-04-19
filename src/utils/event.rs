use rusqlite::{params, Connection, Result};

pub struct Event {
  pub id: Option<i32>,
  pub content: String,
}

impl Event {
  pub fn new(content: String) -> Self {
    Event {
      id: None,
      content: content,
    }
  }
}

impl Event {
  pub fn into_database(&self, conn: &Connection) -> Result<()> {
    conn.execute(
      "INSERT INTO events (summary, done) values (?1, ?2)",
      params![self.content],
    )?;
    Ok(())
  }
}
