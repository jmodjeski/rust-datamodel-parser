#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::io::stdin;
use std::process::exit;

use clap::App;

mod datamodel_parser;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let mut source = String::new();

    if matches.is_present("stdin") {
      match stdin().read_to_string(&mut source) {
        Ok(r) => println!("Read {} bytes from STDIN", r),
        Err(e) => {
          println!("Error reading from STDIN {}", e);
          exit(1);
        }
      };
    } else {
      match matches.value_of("data_model") {
        Some(v) => {
          match File::open(Path::new(&v)) {
            Err(e) => {
              println!("Invalid data-model file: {}", e);
              exit(1);
            },
            Ok(mut f) => 
              match f.read_to_string(&mut source) {
                Err(e) => {
                  println!("Invalid data-model file {}", e);
                  exit(1);
                },
                Ok(v) => {
                  println!("read {} bytes", v, );
                }
              }
          }
        },
        None => {
          println!("arguments error --stdin or --data-model must be present");
          exit(1)
        }
      };
    }

    match datamodel_parser::models(&source) {
        Ok(r) => println!("Parsed as: {}", serde_json::to_string_pretty(&r).unwrap()),
        Err(e) => println!("Parse error: {}", e),
    }
}
