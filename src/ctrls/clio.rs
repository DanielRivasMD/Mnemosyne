use clap::ArgMatches;

pub fn mnemosyne_clio(mt: &ArgMatches) {
  println!("inside clio");
  if let Some(_matches) = mt.subcommand_matches("prueba") {
    println!("nested prueba");
  }
}
