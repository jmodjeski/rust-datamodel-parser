// temporary, need to figure out why applying this before the include doesn't work
#![cfg_attr(feature = "cargo-clippy", allow(suspicious_else_formatting, single_match, cyclomatic_complexity, unit_arg, naive_bytecount, len_zero))]

#[derive(Serialize)]
pub struct DataModelTypeDeclaration {
    pub name: String,
    pub fields: Vec<DataModelFieldDeclaration>,
}

#[derive(Serialize)]
pub struct DataModelFieldDeclaration {
    pub name: String,
    pub field_type: DataModelTypeRef,
    pub directives: Vec<DataModelFieldDirective>,
}

#[derive(Serialize)]
pub struct DataModelTypeRef {
    pub name: String,
    pub inner_type: Option<Box<DataModelTypeRef>>,
    pub required: bool,
}

#[derive(Serialize)]
pub struct DataModelFieldDirective {
    pub name: String,
    pub arguments: Vec<DataModelFieldDirectiveArg>,
}

#[derive(Serialize)]
pub struct DataModelFieldDirectiveArg {
    pub name: String,
    pub value: String,
    pub quoted: bool,
}

// need to figure out why this doesn't work.
// #[cfg_attr(feature = "cargo-clippy", allow(suspicious_else_formatting, single_match, cyclomatic_complexity, unit_arg, naive_bytecount, len_zero))]
include!(concat!(env!("OUT_DIR"), "/datamodel_grammar.rs"));
