use clap::ArgMatches;
use rusqlite::{Connection, Result};

use crate::utils::event::Event;

//#[allow(unused_variables)]
pub fn mnemosyne_new(mt: &ArgMatches, conn: &Connection) {
  println!("inside new");
  let mess = String::from("new message");
  add_event(conn, mess);
}

pub fn add_event(conn: &Connection, summ: String) -> Result<()> {
  Event::new(summ.clone()).into_database(&conn)?;
  //if cfg.verbose {
  println!(
    "added event with summary {}\" to database",
    //backticks_or_quotes(cfg.backticks, true),
    summ
  );
  //}
  Ok(())
}
