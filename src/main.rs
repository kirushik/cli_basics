#[macro_use]
extern crate clap;

#[macro_use]
extern crate log;
extern crate flexi_logger;

use clap::{Arg, App, SubCommand, AppSettings};

fn main() {
  let matches = App::new("cli_basics")
                    .version("0.1")
                    .author("Kirill Pimenov <kirill@pimenov.cc>")
                    .about("Sample REST API client in Rust")
                    .arg(Arg::with_name("debug")
                             .short("d")
                             .help("Enables debugging output"))
                    .subcommand(SubCommand::with_name("get")
                                           .help("Launch a GET request to a JSON endpoint"))
                    .setting(AppSettings::SubcommandRequiredElseHelp)
                    .get_matches();

  if matches.is_present("debug") {
    flexi_logger::init(flexi_logger::LogConfig::new(), Some("cli_basics=debug".to_string())).unwrap();
  };

  debug!("End of the app!");
}
