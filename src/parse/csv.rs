use serde_json::{Map, Value};
use std::io::{BufRead, BufReader, Read};

use crate::parse::{
    field::collect_fields,
    validate::validate_field,
    yaml::{DataType, FieldSpec, Schema},
};

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

            let value = validate_field(field, spec)
                .or_else(|| spec.default_value.clone())
                .ok_or_else(|| ParseError::MissingField(spec.name.clone()))?;

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
