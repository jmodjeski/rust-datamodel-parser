#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::env;
use std::fs::File;
use std::io::{Read};
use std::path::Path;

#[derive(Serialize)]
pub struct DataModelTypeDeclaration<'a> {
    name: &'a str,
    fields: Vec<DataModelFieldDeclaration<'a>>,
}

#[derive(Serialize)]
pub struct DataModelFieldDeclaration<'a> {
    name: &'a str,
    field_type: &'a str,
    required: bool,
    directives: Vec<DataModelFieldDirective<'a>>
}

#[derive(Serialize)]
pub struct DataModelFieldDirective<'a> {
    name: &'a str,
    arguments: Vec<DataModelFieldDirectiveArg<'a>>,
}

#[derive(Serialize)]
pub struct DataModelFieldDirectiveArg<'a> {
    name: &'a str,
    value: String,
    quoted: bool,
}

mod my_grammar {
    include!(concat!(env!("OUT_DIR"), "/my_grammar.rs"));
}

fn main() {
	let args = env::args_os().collect::<Vec<_>>();
	let mut source = String::new();
    File::open(Path::new(&args[1])).unwrap().read_to_string(&mut source).unwrap();

    match my_grammar::models(&source) {
        Ok(r) => println!("Parsed as: {}", serde_json::to_string_pretty(&r).unwrap()),
        Err(e) => println!("Parse error: {}", e),
    }
}
