////////////////////////////////////////////////////////////////////////////////////////////////////

/// Mnenosyne wrapper
use mnemosyne::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

use anyhow::Result as anyResult;
use rusqlite::NO_PARAMS;
use rusqlite::{Connection, Result};

////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::utils::cli::cli_mnemosyne;

////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() -> anyResult<()> {
  ////////////////////////////////////////////////////////////////////////////////////////////////////

  let matches = cli_mnemosyne().get_matches();

  // TODO: check database exits ? update : create
  // TODO: use commands to manipulate database
  // TODO: build interfase
  // TODO: add tests
  // TODO: error handlers

  let path = String::from("/Users/drivas/.mnemosyne.db");
  let conn = Connection::open(path)?;
  up(&conn)?;

  if let Some(matches) = matches.subcommand_matches("new") {
    ctrls::new::mnemosyne_new(matches, &conn);
    // ctrls::clio::mnemosyne_clio(matches)?;
  }

  if let Some(matches) = matches.subcommand_matches("list") {
    ctrls::list::mnemosyne_ls(matches, &conn);
    // ctrls::clio::mnemosyne_clio(matches)?;
  }

  //if let Some(matches) = matches.subcommand_matches("clio") {
  //ctrls::clio::mnemosyne_clio(matches);
  //// ctrls::clio::mnemosyne_clio(matches)?;
  //}

  Ok(())
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
