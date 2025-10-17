use std::collections::HashMap;

use serde_json::{Map, Number, Value};

use crate::{
    parse::yaml::{DataType, FieldSpec},
    types::country::parse_country_code,
    utils::clean_line,
};

pub fn collect_fields(line: &str, sep: &str) -> Vec<String> {
    let fields: Vec<String> = line.split(sep).map(|x| clean_line(x).to_string()).collect();
    fields
}

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

// TODO: return optional type and remove invalid entries from schema
pub fn csv_line_to_payment(
    line: &str,
    sep: &str,
    headers: &Vec<String>,
    schema: &HashMap<String, FieldSpec>,
) -> Option<Value> {
    let mut obj = Map::new();

    let fields = collect_fields(line, sep);

    // TODO: potentially loop over spec to ensure all fields are present
    for (header, field) in headers.iter().zip(&fields) {
        let spec = match schema.get(header) {
            Some(s) => s,
            None => continue,
        };
        let value = match validate_field(field, spec) {
            Some(x) => x,
            _ => return None,
        };
        obj.insert(spec.name.to_string(), value);
    }

    Some(Value::Object(obj))
}
