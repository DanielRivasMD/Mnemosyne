use clap::ArgMatches;
use rusqlite::NO_PARAMS;
use rusqlite::{params, Connection, Result};
use termion::terminal_size;

use crate::utils::event::Event;

//#[allow(unused_variables)]
pub fn mnemosyne_ls(mt: &ArgMatches, conn: &Connection) -> Result<()> {
  println!("inside list");

  if let Some(ids) = mt.value_of("id") {
    match parse_ids(ids) {
      Ok(id) => {
        list_event_by_id(&conn, id);
      }
      Err(_) => {
        eprintln!("Invalid input. Try doing something like `work ls <id of an event>`");
      }
    }
  } else {
    list_all_events(&conn)?;
  }
  Ok(())
}

fn parse_ids(id_string: &str) -> Result<u32, std::num::ParseIntError> {
  id_string.parse::<u32>()
}

pub fn list_event_by_id(conn: &Connection, id: u32) -> Result<()> {
  let event_result = get_event_by_id(&conn, id);
  let event = match event_result {
    Ok(x) => x,
    Err(_) => {
      eprintln!("Error. Try another id that is valid");
      return Ok(());
    }
  };
  print_vec_events(&vec![event]);
  Ok(())
}

pub fn get_event_by_id(conn: &Connection, id_to_query: u32) -> Result<Event> {
  conn.query_row(
    "SELECT id, summary, done FROM events WHERE id = ?1",
    params![Some(id_to_query)],
    |row| {
      Ok(Event {
        id: row.get(0)?,
        content: row.get(1)?,
      })
    },
  )
}

pub fn list_all_events(conn: &Connection) -> Result<()> {
  let events_special_stuff = get_all_events(&conn);
  let events = match events_special_stuff {
    Ok(x) => x,
    Err(_) => {
      eprintln!("An Error Occured.\nTry adding some events to list first.\nWhoops");
      return Ok(());
    }
  };
  let mut events_out: Vec<Event> = Vec::new();
  for event_res in events {
    let _event = match event_res {
      Ok(x) => x,
      Err(_) => {
        eprintln!("Error Occured. Whoops.");
        return Ok(());
      }
    };
    events_out.push(_event);
  }
  print_vec_events(&events_out);
  Ok(())
}

pub fn get_all_events(conn: &Connection) -> Result<Vec<Result<Event>>> {
  let mut stmt = conn.prepare("SELECT id, summary, done FROM events")?;
  let rows = stmt.query_map(NO_PARAMS, |row| {
    Ok(Event {
      id: Some(row.get(0)?),
      content: row.get(1)?,
    })
  })?;
  let mut events = Vec::new();
  for event in rows {
    events.push(event);
  }
  Ok(events)
}
fn print_vec_events(event_vec: &Vec<Event>) {
  line('_');
  println!("DONE | ID | SUMMARY");
  line('-');
  for event in event_vec {
    println!("{} | {}", event.id.unwrap(), event.content);
  }
  line('-');
}

fn line(char_to_repeat: char) {
  let size = terminal_size().unwrap();
  let mut size_str = String::new();
  for _ in 0..size.0 {
    size_str.push(char_to_repeat);
  }
  println!("{}", size_str);
}
