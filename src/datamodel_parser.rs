// temporary, need to figure out why applying this before the include doesn't work
#![cfg_attr(feature = "cargo-clippy", allow(suspicious_else_formatting, single_match, cyclomatic_complexity, unit_arg, naive_bytecount, len_zero))]

#[derive(Serialize)]
pub struct DataModelTypeDeclaration<'a> {
    pub name: &'a str,
    pub fields: Vec<DataModelFieldDeclaration<'a>>,
}

#[derive(Serialize)]
pub struct DataModelFieldDeclaration<'a> {
    pub name: &'a str,
    pub field_type: &'a str,
    pub required: bool,
    pub directives: Vec<DataModelFieldDirective<'a>>,
}

#[derive(Serialize)]
pub struct DataModelFieldDirective<'a> {
    pub name: &'a str,
    pub arguments: Vec<DataModelFieldDirectiveArg<'a>>,
}

#[derive(Serialize)]
pub struct DataModelFieldDirectiveArg<'a> {
    pub name: &'a str,
    pub value: String,
    pub quoted: bool,
}

// need to figure out why this doesn't work.
// #[cfg_attr(feature = "cargo-clippy", allow(suspicious_else_formatting, single_match, cyclomatic_complexity, unit_arg, naive_bytecount, len_zero))]
include!(concat!(env!("OUT_DIR"), "/datamodel_grammar.rs"));
