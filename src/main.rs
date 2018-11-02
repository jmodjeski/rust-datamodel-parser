#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::env;
use std::fs::File;
use std::io::{Read};
use std::path::Path;

mod ast_types {
    include!("./ast_types.rs");
}

mod datamodel_grammar {
    use ast_types::DataModelTypeDeclaration;
    use ast_types::DataModelFieldDeclaration;
    use ast_types::DataModelFieldDirective;
    use ast_types::DataModelFieldDirectiveArg;
    include!(concat!(env!("OUT_DIR"), "/datamodel_grammar.rs"));
}

fn main() {
	let args = env::args_os().collect::<Vec<_>>();
	let mut source = String::new();
    File::open(Path::new(&args[1])).unwrap().read_to_string(&mut source).unwrap();

    match datamodel_grammar::models(&source) {
        Ok(r) => println!("Parsed as: {}", serde_json::to_string_pretty(&r).unwrap()),
        Err(e) => println!("Parse error: {}", e),
    }
}
