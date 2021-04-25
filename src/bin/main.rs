////////////////////////////////////////////////////////////////////////////////////////////////////

/// Mnenosyne wrapper
use mnemosyne::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

use anyhow::Result as anyResult;
use rusqlite::NO_PARAMS;
//use rusqlite::{Connection, Result};

use rusqlite::{params, Connection, Result};

use std::env::var;
use std::path::PathBuf;

////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::utils::cli::cli_mnemosyne;
use crate::utils::event;

////////////////////////////////////////////////////////////////////////////////////////////////////

//#[derive(Debug)]
//struct Person {
//id: i32,
//name: String,
//data: Option<Vec<u8>>,
//}

fn main() -> anyResult<()> {
  ////////////////////////////////////////////////////////////////////////////////////////////////////
  //open connection to database
  let home = var("HOME").unwrap();
  let path_to_db: PathBuf = [home.as_str(), ".mnemosyne.db"].iter().collect();

  //if !path_to_db.exists() {
  //std::fs::create_dir_all(path_to_db).unwrap();
  //}

  let conn = Connection::open(path_to_db)?;
  //conn.execute(
  //"CREATE TABLE person (
  //id              INTEGER PRIMARY KEY,
  //name            TEXT NOT NULL,
  //data            BLOB
  //)",
  //[],
  //)?;
  //let me = Person {
  //id: 0,
  //name: "Steven".to_string(),
  //data: None,
  //};
  //conn.execute(
  //"INSERT INTO person (name, data) VALUES (?1, ?2)",
  //params![me.name, me.data],
  //)?;
  //let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
  //let person_iter = stmt.query_map([], |row| {
  //Ok(Person {
  //id: row.get(0)?,
  //name: row.get(1)?,
  //data: row.get(2)?,
  //})
  //})?;

  //for person in person_iter {
  //println!("Found person {:?}", person.unwrap());
  //}

  conn.execute(
    "CREATE TABLE event (
      id        INTEGER PRIMARY KEY,
      content   TEXT NOT NULL
      )",
    [],
  )?;
  println!("table created");

  let ev = event::Event {
    id: 0,
    content: "important".to_string(),
  };

  conn.execute(
    "INSERT INTO event (content) VALUES (?1)",
    params![ev.content],
  )?;
  println!("event inserted");

  let mut stmt = conn.prepare("SELECT id, content FROM event")?;
  let event_iter = stmt.query_map([], |row| {
    Ok(event::Event {
      id: row.get(0)?,
      content: row.get(1)?,
    })
  })?;

  for event in event_iter {
    println!("Event: {:?}", event.unwrap());
  }
  Ok(())

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  //let matches = cli_mnemosyne().get_matches();

  //// TODO: check database exits ? update : create
  //// TODO: use commands to manipulate database
  //// TODO: build interfase
  //// TODO: add tests
  //// TODO: error handlers

  //let path = String::from("/Users/drivas/.mnemosyne.db");
  //let conn = Connection::open(path)?;
  //up(&conn)?;

  //if let Some(matches) = matches.subcommand_matches("new") {
  //ctrls::new::mnemosyne_new(matches, &conn)?;
  //// ctrls::clio::mnemosyne_clio(matches)?;
  //}

  //if let Some(matches) = matches.subcommand_matches("list") {
  //ctrls::list::mnemosyne_ls(matches, &conn)?;
  //// ctrls::clio::mnemosyne_clio(matches)?;
  //}

  ////if let Some(matches) = matches.subcommand_matches("clio") {
  ////ctrls::clio::mnemosyne_clio(matches);
  ////// ctrls::clio::mnemosyne_clio(matches)?;
  ////}

  //Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn up(conn: &Connection) -> Result<()> {
  conn.execute(
    "create table if not exists events (
     id integer primary key autoincrement,
    summary text not null,
    done bool not null
         )",
    NO_PARAMS,
  )?;
  Ok(())
}
