use std::env;
use std::fs::File;
use std::io::{Read};
use std::path::Path;

pub struct DataModelTypeDeclaration<'a> {
    name: &'a str,
    field: &'a DataModelFieldDeclaration<'a>,
}

pub struct DataModelFieldDeclaration<'a> {
    name: &'a str,
    fieldType: &'a str,
}

mod my_grammar {
    include!(concat!(env!("OUT_DIR"), "/my_grammar.rs"));
}

fn main() {
	let args = env::args_os().collect::<Vec<_>>();
	let mut source = String::new();
    File::open(Path::new(&args[1])).unwrap().read_to_string(&mut source).unwrap();

    match my_grammar::modelType(&source) {
        Ok(r) => println!("Parsed as: {}", r.name),
        Err(e) => println!("Parse error: {}", e),
    }
}
