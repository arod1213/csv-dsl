use serde_json::{Map, Number, Value};
use std::io::{BufRead, BufReader, Read};

use crate::{
    parse::{
        field::collect_fields,
        yaml::{DataType, FieldSpec, Schema},
    },
    types::country::parse_country_code,
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
    EOF,
}

pub struct CSVParser<'a, R: std::io::Read + 'a> {
    reader: &'a mut BufReader<R>,
    schema: &'a Schema,
    headers: Vec<String>,
    sep: &'a char,
}

impl<'a, R: Read> CSVParser<'a, R> {
    pub fn new(reader: &'a mut BufReader<R>, schema: &'a Schema, sep: &'a char) -> Self {
        let line = CSVParser::read_line(reader).unwrap();
        let headers = collect_fields(&line, &sep);

        CSVParser {
            reader,
            schema,
            headers,
            sep,
        }
    }
    fn read_line(reader: &mut BufReader<R>) -> Option<String> {
        let mut buf: String = "".to_string();
        let byte_count = match reader.read_line(&mut buf) {
            Ok(x) if x == 0 => return None,
            Ok(x) => x,
            Err(_) => return None,
        };
        Some(buf[0..byte_count].to_string())
    }

    fn next_line(&mut self) -> Option<String> {
        Self::read_line(self.reader)
    }

    pub fn next(&mut self) -> Result<Value, ParseError> {
        let line = match self.next_line() {
            Some(x) => x,
            None => return Err(ParseError::EOF),
        };

        let mut obj = Map::new();

        let fields = collect_fields(&line, self.sep);
        for (header, field) in self.headers.iter().zip(&fields) {
            let spec = match self.schema.alias_to_name.get(header) {
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

        let missing: Vec<&FieldSpec> = self
            .schema
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
}
