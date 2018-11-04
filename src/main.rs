#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::fs::File;
use std::io::{Read};
use std::path::Path;

use clap::{App};

mod datamodel_parser;

fn main() {
  let yaml = load_yaml!("cli.yml");
  let matches = App::from_yaml(yaml).get_matches();
  let datamodel_file = matches.value_of("data_model").unwrap();
  let mut source = String::new();
  File::open(Path::new(&datamodel_file)).unwrap().read_to_string(&mut source).unwrap();

  match datamodel_parser::models(&source) {
    Ok(r) => println!("Parsed as: {}", serde_json::to_string_pretty(&r).unwrap()),
    Err(e) => println!("Parse error: {}", e),
  }
}
