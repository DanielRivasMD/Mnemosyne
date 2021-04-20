////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use clap::{crate_authors, crate_version, App, AppSettings};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn cli_mnemosyne() -> App<'static> {
  App::new("mnemosyne")
    .version(crate_version!())
    .author(crate_authors!())
    .setting(AppSettings::ArgRequiredElseHelp)
    .setting(AppSettings::ColoredHelp)
    .about(
      "memory godess

    ",
    )
    //.subcommand(
    //App::new("clio")
    //.about("clio")
    //.setting(AppSettings::ArgRequiredElseHelp)
    //.setting(AppSettings::ColoredHelp)
    //.subcommand(App::new("prueba").about("nested")),
    //)
    .subcommand(App::new("new"))
    .version(crate_version!())
    .author(crate_authors!())
    .about("")
    //.subcommand(App::new("edit"))
    // .version(crate_version!())
    // .author(crate_authors!())
    // .about("")
    .subcommand(App::new("list"))
    .version(crate_version!())
    .author(crate_authors!())
    .about("")
}

////////////////////////////////////////////////////////////////////////////////////////////////////
