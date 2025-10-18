use std::collections::HashMap;

use serde_json::{Map, Number, Value};

use crate::{
    parse::{
        field::collect_fields,
        yaml::{DataType, FieldSpec, Schema},
    },
    types::country::parse_country_code,
    utils::clean_line,
};

fn validate_field(field: &str, spec: &FieldSpec) -> Option<Value> {
    match spec.r#type {
        DataType::Country => {
            let country_name = parse_country_code(field);
            return Some(Value::String(country_name));
        }
        // TODO: provide a more robust format to parse by (01-01-01) would fail atm
        DataType::Date => {
            let fixed = field.replace("-", "/");
            let date = match dateparser::parse(&fixed) {
                Ok(x) => x,
                Err(_) => {
                    if spec.optional {
                        return Some(Value::Null);
                    }
                    return None;
                }
            };
            return Some(Value::String(date.to_string()));
        }
        DataType::String => {
            if spec.optional && field == "" {
                return Some(Value::Null);
            }
            if field == "" {
                return None;
            }
            return Some(Value::String(field.to_string()));
        }
        DataType::Float => {
            let num = match field.parse::<f64>() {
                Ok(x) => x,
                Err(_) => {
                    if spec.optional {
                        return Some(Value::Null);
                    }
                    return None;
                }
            };
            let num = Number::from_f64(num).unwrap();
            return Some(Value::Number(num));
        }
        DataType::Int => {
            let num = match field.parse::<i128>() {
                Ok(x) => x,
                Err(_) => {
                    if spec.optional {
                        return Some(Value::Null);
                    }
                    return None;
                }
            };
            let num = Number::from_i128(num).unwrap();
            return Some(Value::Number(num));
        }
        DataType::Uint => {
            let num = match field.parse::<u128>() {
                Ok(x) => x,
                Err(_) => {
                    if spec.optional {
                        return Some(Value::Null);
                    }
                    return None;
                }
            };
            let num = Number::from_u128(num).unwrap();
            return Some(Value::Number(num));
        }
        _ => return None,
    }
}

#[derive(Debug)]
pub struct FieldInfo {
    field_name: String,
    field_type: DataType,
    value: String,
}

#[derive(Debug)]
pub enum ParseError {
    BadField(FieldInfo),
    MissingField(String),
}
// TODO: return optional type and remove invalid entries from schema
pub fn csv_line_to_payment(
    line: &str,
    sep: &char,
    headers: &Vec<String>,
    schema: &Schema,
) -> Result<Value, ParseError> {
    let mut obj = Map::new();

    let fields = collect_fields(line, sep);

    // TODO: potentially loop over spec to ensure all fields are present
    for (header, field) in headers.iter().zip(&fields) {
        let spec = match schema.alias_to_name.get(header) {
            Some(s) => s,
            None => continue,
        };
        let value = match validate_field(field, spec) {
            Some(x) => x,
            _ => {
                return Err(ParseError::BadField(FieldInfo {
                    value: field.to_string(),
                    field_name: spec.name.clone(),
                    field_type: spec.r#type.clone(),
                }));
            }
        };
        obj.insert(spec.name.to_string(), value);
    }

    let missing: Vec<&FieldSpec> = schema
        .specs
        .iter()
        .filter(|s| !obj.get(&s.name).is_some())
        .collect();
    for field in missing.iter() {
        if field.optional {
            obj.insert(field.name.to_string(), Value::Null);
            continue;
        } else {
            return Err(ParseError::MissingField(field.name.to_string()));
        }
    }

    Ok(Value::Object(obj))
}
