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
        csv::{CSVParser, ParseError},
        yaml::Schema,
    },
    read::get_path,
};

fn read_file(name: &str, args: &Args) {
    let path = match get_path(name) {
        Some(s) => s,
        None => return,
    };

    let schema = Schema::new(&args.schema);
    let file = File::open(path).expect("could not open csv");
    let mut reader = BufReader::new(file);

    let mut parser = CSVParser::new(&mut reader, &schema, &args.separator);

    let mut statements: Vec<Value> = vec![];
    let mut line_num: usize = 0;
    loop {
        let obj = match parser.next() {
            Ok(x) => x,
            Err(e) => {
                if let ParseError::EOF = e {
                    break;
                }

                if args.strict {
                    match e {
                        ParseError::BadField(s) => {
                            eprintln!("bad field in {:?} at {}: {:?}", name, line_num, s);
                        }
                        ParseError::MissingField(s) => {
                            eprintln!("missing field in {:?} at {}: {:?}", name, line_num, s);
                        }
                        _ => (),
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
    if !args.strict {
        println!("{}", serde_json::to_string_pretty(&json).unwrap());
    }
}

fn main() {
    let args = Args::parse();

    for name in &args.filepaths {
        read_file(name, &args);
    }
}
