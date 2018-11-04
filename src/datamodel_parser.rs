
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

#[cfg_attr(feature = "cargo-clippy", allow(clippy_all))]
include!(concat!(env!("OUT_DIR"), "/datamodel_grammar.rs"));
