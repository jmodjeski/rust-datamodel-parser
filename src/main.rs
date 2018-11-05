#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;

use std::fs::File;
use std::fs::OpenOptions;
use std::io::stdin;
use std::io::stdout;
use std::io::{Read, Write};
use std::path::Path;
use std::process::exit;

use clap::App;

mod datamodel_parser;
mod json_formatter;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let model_source: Result<String, (std::io::Error, String)> =
        match matches.value_of("data_model") {
            Some(path) => read_datamodel(path),
            None => read_stdin(),
        };
    let source: String = match model_source {
        Ok(k) => k,
        Err(e) => {
            eprintln!("Error reading data model from '{}': {}", e.1, e.0);
            exit(1);
        }
    };

    let model = match datamodel_parser::models(&source) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Parse error: {}", e);
            exit(1);
        }
    };

    let formatted = match matches.value_of("format") {
        Some(s) => match s {
            "json" => {
                let format_options = json_formatter::JSONFormatterOptions {
                    pretty: matches.is_present("format_json_pretty"),
                };
                match json_formatter::format(format_options, model) {
                    Ok(buf) => buf,
                    Err(e) => {
                        eprintln!("Format error: {}", e);
                        exit(1);
                    }
                }
            }
            unk => panic!("unknown formatter: {}", unk),
        },
        None => panic!("Formatter should not reach this point."),
    };

    let output_result: Result<usize, (std::io::Error, String)> = match matches.value_of("output") {
        Some(path) => write_output(path, formatted),
        None => write_stdout(formatted),
    };

    match output_result {
        Ok(u) => u,
        Err(e) => {
            eprintln!("error writing to '{}': {}", e.1, e.0);
            exit(1);
        }
    };
    exit(0);
}

fn read_stdin() -> Result<String, (std::io::Error, String)> {
    let mut source = String::new();
    match stdin().read_to_string(&mut source) {
        Ok(s) => s,
        Err(e) => return Err((e, "STDIN".to_owned())),
    };
    Ok(source)
}

fn read_datamodel(path: &str) -> Result<String, (std::io::Error, String)> {
    let mut source = String::new();
    let resolved_path = Path::new(&path);
    let mut file = match File::open(resolved_path) {
        Ok(f) => f,
        Err(e) => return Err((e, path.to_owned())),
    };
    match file.read_to_string(&mut source) {
        Ok(s) => s,
        Err(e) => return Err((e, path.to_owned())),
    };
    Ok(source)
}

fn write_stdout(source: String) -> Result<usize, (std::io::Error, String)> {
    match stdout().write(&source.into_bytes()) {
        Ok(s) => Ok(s),
        Err(e) => Err((e, "STDOUT".to_owned())),
    }
}

fn write_output(path: &str, source: String) -> Result<usize, (std::io::Error, String)> {
    let resolved_path = Path::new(&path);
    let mut file = match OpenOptions::new()
        .create(true)
        .write(true)
        .open(resolved_path)
    {
        Ok(f) => f,
        Err(e) => return Err((e, path.to_owned())),
    };

    match file.write(&source.into_bytes()) {
        Ok(s) => Ok(s),
        Err(e) => Err((e, path.to_owned())),
    }
}
