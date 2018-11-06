use datamodel_parser::RootTypes;
use datamodel_parser::DataModelTypeRef;
use datamodel_parser::DataModelFieldDeclaration;
use std::string::String;
use std::fmt::{Write, Error};
use std::collections::HashMap;

pub struct DOTFormatterOptions {}

struct Port {
    left: Option<String>,
    right: Option<String>,
}

pub fn format(
    _options: DOTFormatterOptions,
    types: Vec<RootTypes>,
) -> Result<String, Error> {
    let mut value = String::new();
    let mut ports: HashMap<String, Port> = HashMap::new();

    value.write_str("digraph model {\r\n")?;

    let type_iter = types.iter();
    for val in type_iter {
        match val {
            RootTypes::Scalar(_s) => {
                // no op
            },
            RootTypes::Type(t) => {
                value.write_str(&format!("\ttable{} [\r\n", t.name))?;
                value.write_str("\t\tshape=plaintext\r\n")?;
                value.write_str("\t\tlabel=<\r\n")?;
                value.write_str("\t\t\t<table border='0' cellborder='1' cellspacing='0'>\r\n")?;
                value.write_str(&format!("\t\t\t\t<tr><td colspan='2'>{}</td></tr>\r\n", t.name))?;

                let field_iter = t.fields.iter();
                for field in field_iter {
                    let field_type = get_field_type(&field.field_type);
                    let relation = get_relation_directive(&field);
                    match relation {
                        None => value.write_str(&format!("\t\t\t\t<tr><td>{}</td><td>{}</td></tr>\r\n", field.name, field_type))?,
                        Some(relation_name) => {
                            let port_name = relation_name.clone();
                            let entry_name = relation_name.clone();
                            let root_type = get_root_field_type(&field.field_type);
                            let mut port = ports.entry(entry_name).or_insert(Port{
                                left: None,
                                right: None
                            });
                            if field_type.starts_with("[") {
                                port.right = Some(format!("table{}", root_type));
                            } else {
                                port.left = Some(format!("table{}", root_type));
                            }
                            value.write_str(&format!("\t\t\t\t<tr><td port='{}'>{}</td><td>{}</td></tr>\r\n", port_name, field.name, field_type))?;
                        }
                    }
                    
                }

                value.write_str("\t\t\t</table>")?;
                value.write_str(">\t];\r\n")?;
            }
        }
    }

    let port_iter = ports.iter();
    for port_entry in port_iter {
        let port = &port_entry.1;
        let port_name = port_entry.0;
        let left_side = match port.left {
            Some(ref v) => v.clone(),
            None => "<missing>".to_string()
        };
        let right_side = match port.right {
            Some(ref v) => v.clone(),
            None => "<missing>".to_string()
        };
        value.write_str(&format!("\t{}:{} -> {}:{};\r\n", left_side, port_name, right_side, port_name))?;
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

fn get_root_field_type(field: &DataModelTypeRef) -> String {
    let ref reffed = field.inner_type;
    match reffed {
        Some(v) => get_root_field_type(v.as_ref()),
        None => field.name.to_owned(),
    }
}

fn get_relation_directive(field: &DataModelFieldDeclaration) -> Option<String> {
    let directive_iter = field.directives.iter();
    for directive in directive_iter {
        if directive.name == "relation" {
            let arg_iter = directive.arguments.iter();
            for arg in arg_iter {
                if arg.name == "name" {
                    return Some(arg.value.to_owned());
                }
            }
        }
    };
    None
}
