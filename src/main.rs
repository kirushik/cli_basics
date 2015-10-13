#[macro_use]
extern crate clap;
use clap::{Arg, App, SubCommand};

#[macro_use]
extern crate log;
extern crate flexi_logger;

extern crate hyper;
use hyper::client::Client;
use std::io::Read;

extern crate rustc_serialize;
use rustc_serialize::json::Json;

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
                    .subcommand_required_else_help(true)
                    .get_matches();

  if matches.is_present("debug") {
    flexi_logger::init(flexi_logger::LogConfig::new(), Some("cli_basics=debug".to_string())).unwrap();
  };

  match matches.subcommand_name() {
    Some("get") => run_get(),
    _ => unreachable!()
  };

  debug!("End of the app!");
}

fn run_get() {
  debug!("Running GET request");

  let client = Client::new();
  let mut response = client.get("https://eu.httpbin.org/get").send().unwrap();

  debug!("Response status was {:?}", response.status);

  let mut body = String::new();
  response.read_to_string(&mut body).unwrap();

  let data = Json::from_str(&body).unwrap();

  println!("{:?}", data);

  debug!("Done running the request");
}
