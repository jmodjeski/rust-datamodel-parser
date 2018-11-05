use datamodel_parser::DataModelTypeDeclaration;
use datamodel_parser::DataModelTypeRef;
use std::string::String;
use std::fmt::{Write, Error};

pub struct DOTFormatterOptions {}

pub fn format(
    _options: DOTFormatterOptions,
    types: Vec<DataModelTypeDeclaration>,
) -> Result<String, Error> {
    let mut value = String::new();

    value.write_str("digraph model {\r\n")?;

    let type_iter = types.iter();
    for val in type_iter {
        value.write_str(&format!("\ttable{} [\r\n", val.name))?;
        value.write_str("\t\tshape=plaintext\r\n")?;
        value.write_str("\t\tlabel=<\r\n")?;
        value.write_str("\t\t\t<table border='0' cellborder='1' cellspacing='0'>\r\n")?;
        value.write_str(&format!("\t\t\t\t<tr><td colspan='2'>{}</td></tr>\r\n", val.name))?;

        let field_iter = val.fields.iter();
        for field in field_iter {
            let t = get_field_type(&field.field_type);
            value.write_str(&format!("\t\t\t\t<tr><td>{}</td><td>{}</td></tr>\r\n", field.name, t))?;
        }

        value.write_str("\t\t\t</table>")?;
        value.write_str(">\t]\r\n")?;
    }

    value.write_str("}\r\n")?;

    Ok(value.to_owned())
}

fn get_field_type(field: &DataModelTypeRef) -> String {
    let ref reffed = field.inner_type;
    let required;
    if field.required { required = "!"; } else { required = ""; };
    match reffed {
        Some(v) => format!("[{}]{}", get_field_type(v.as_ref()), required),
        None => format!("{}{}", field.name, required),
    }
}