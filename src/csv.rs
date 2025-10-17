use std::collections::HashMap;

use serde_json::{Map, Number, Value};

use crate::{utils::clean_line, yaml::FieldSpec};

#[derive(Debug, PartialEq, Eq)]
enum DigitType {
    Integer,
    Float,
}

fn is_digit(text: &str) -> Option<DigitType> {
    if text.len() == 0 {
        return None;
    };

    let mut decimal_count: usize = 0;
    for (i, c) in text.chars().enumerate() {
        if c == '-' && (i != 0 || text.len() < 2) {
            return None;
        };

        if c == '.' {
            decimal_count += 1;
            if decimal_count > 1 {
                return None;
            }
            continue;
        }

        if !c.is_digit(10) && c != '-' {
            return None;
        }
    }
    if decimal_count != 0 {
        return Some(DigitType::Float);
    }
    Some(DigitType::Integer)
}

fn field_to_value(field: &str) -> Value {
    if let Some(digit_type) = is_digit(field) {
        match digit_type {
            DigitType::Float => {
                let digit = field.parse::<f64>().unwrap();
                return Value::Number(Number::from_f64(digit).unwrap());
            }
            DigitType::Integer => {
                let digit = field.parse::<i128>().unwrap();
                return Value::Number(Number::from_i128(digit).unwrap());
            }
        };
    }
    match field {
        "true" => return Value::Bool(true),
        "false" => return Value::Bool(false),
        "-" | "null" | "" => return Value::Null,
        _ => (),
    }
    Value::String(field.to_string())
}

pub fn collect_fields(line: &str, sep: &str) -> Vec<String> {
    let fields: Vec<String> = line.split(sep).map(|x| clean_line(x).to_string()).collect();
    fields
}

pub fn csv_line_to_json(line: &str, sep: &str, headers: &Vec<String>) -> Value {
    let mut obj = Map::new();

    let fields = collect_fields(line, sep);

    for (header, field) in headers.iter().zip(&fields) {
        obj.insert(header.to_string(), field_to_value(&field));
    }

    Value::Object(obj)
}

fn validate_field(field: &str, spec: &FieldSpec) -> Option<Value> {
    match spec.r#type.as_str() {
        "string" => {
            if spec.optional && field == "" {
                return Some(Value::Null);
            }
            if field == "" {
                return None;
            }
            return Some(Value::String(field.to_string()));
        }
        "float" => {
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
        "int" => {
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
        "uint" => {
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
