use clap::Parser;
use serde_json::Value;
use std::{fs::File, io::BufReader};

mod cli;
mod parse;
mod read;
mod types;
mod utils;

use crate::{
    cli::Args,
    parse::{
        csv::{ParseError, collect_fields, csv_line_to_payment},
        yaml::yaml_schema,
    },
    read::{get_path, read_line},
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

        let mut line_num: usize = 0;
        loop {
            let line = match read_line(&mut reader) {
                Some(s) => s,
                None => break,
            };
            let obj = match csv_line_to_payment(&line, &args.separator, &headers, &yaml_schema) {
                Ok(x) => x,
                Err(e) => {
                    if args.strict {
                        match e {
                            ParseError::BadField(s) => {
                                panic!("bad field in {:?} at {}: {:?}", name, line_num, s);
                            } // _ => panic!("{:?}", e),
                        }
                    }
                    line_num += 1;
                    continue;
                }
            };
            line_num += 1;
            statements.push(obj);
        }
        let json = Value::from(statements);
        println!("{}", serde_json::to_string_pretty(&json).unwrap());
    }
}
