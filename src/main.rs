#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::fs::File;
use std::fs::OpenOptions;
use std::io::stdin;
use std::io::stdout;
use std::io::{Read, Write};
use std::path::Path;
use std::process::exit;

use clap::App;

mod datamodel_parser;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let mut source = String::new();

    if matches.is_present("stdin") {
        let x = read_stdin(&mut source);
        if x.is_err() {
            eprintln!("Error reading from STDIN {}", x.err().unwrap());
            exit(1);
        }
    } else {
        let v = matches.value_of("data_model").unwrap();
        let x = read_datamodel(&v, &mut source);
        if x.is_err() {
            eprintln!("error reading from {}: {}", v, x.err().unwrap());
            exit(1);
        }
    }

    match datamodel_parser::models(&source) {
        Ok(r) => {
            match serde_json::to_string_pretty(&r) {
                Ok(buf) => {
                    if matches.is_present("stdout") {
                        let x = write_stdout(buf);
                        if x.is_err() {
                            eprintln!("Error writing to STDOUT {}", x.err().unwrap());
                            exit(1);
                        }
                    } else {
                        let v = matches.value_of("output").unwrap();
                        let x = write_output(&v, buf);
                        if x.is_err() {
                            eprintln!("error writing to {}: {}", v, x.err().unwrap());
                            exit(1);
                        }
                    }
                },
                Err(e) => eprintln!("Format error: {}", e),
            }
        },
        Err(e) => eprintln!("Parse error: {}", e),
    }
}

fn read_stdin(source: &mut String) -> std::io::Result<()> {
    stdin().read_to_string(&mut *source)?;
    Ok(())
}

fn read_datamodel(path: &str, source: &mut String) -> std::io::Result<()> {
    let resolved_path = Path::new(&path);
    let mut file = File::open(resolved_path)?;
    file.read_to_string(&mut *source)?;
    Ok(())
}

fn write_stdout(source: String) -> std::io::Result<()> {
    // stdin().read_to_string(&mut *source)?;
    stdout().write(&source.into_bytes())?;
    Ok(())
}

fn write_output(path: &str, source: String) -> std::io::Result<()> {
    let resolved_path = Path::new(&path);
    let mut file = OpenOptions::new().create(true).write(true).open(resolved_path)?;
    file.write(&source.into_bytes())?;
    Ok(())
}
