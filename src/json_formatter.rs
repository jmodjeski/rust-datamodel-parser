extern crate serde_json;

use datamodel_parser::RootTypes;

pub struct JSONFormatterOptions {
    pub pretty: bool,
}

pub fn format(
    options: JSONFormatterOptions,
    types: Vec<RootTypes>,
) -> Result<String, String> {
    if options.pretty {
        match serde_json::to_string_pretty(&types) {
            Ok(f) => return Ok(f),
            Err(e) => return Err(e.to_string()),
        }
    } else {
        match serde_json::to_string(&types) {
            Ok(f) => return Ok(f),
            Err(e) => return Err(e.to_string()),
        }
    }
}
