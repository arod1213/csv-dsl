use clap::Parser;
use serde_json::Value;
use std::{fs::File, io::BufReader};

mod cli;
mod csv;
mod read;
mod utils;
mod yaml;

use crate::{
    cli::Args,
    csv::{collect_fields, csv_line_to_payment},
    read::{get_path, read_line},
    yaml::yaml_schema,
};

fn main() {
    let args = Args::parse();

    for name in &args.filepaths {
        let path = match get_path(name) {
            Some(s) => s,
            None => continue,
        };

        let file = File::open(path).expect("could not open csv");
        let mut reader = BufReader::new(file);

        let line = read_line(&mut reader).unwrap();
        let headers = collect_fields(&line, &args.separator);

        let mut statements: Vec<Value> = vec![];
        let yaml_schema = yaml_schema(&args.schema);
        loop {
            let line = match read_line(&mut reader) {
                Some(s) => s,
                None => break,
            };
            let obj = match csv_line_to_payment(&line, &args.separator, &headers, &yaml_schema) {
                Some(x) => x,
                None => continue,
            };
            statements.push(obj);
        }
        let json = Value::from(statements);
        println!("{}", serde_json::to_string_pretty(&json).unwrap());
    }
}
