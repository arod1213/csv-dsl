use serde_json::{Number, Value};

use crate::{
    parse::yaml::{DataType, FieldSpec},
    types::country::parse_country_code,
};

pub fn validate_field(field: &str, spec: &FieldSpec) -> Option<Value> {
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
