use clap::Parser;
use std::{
    fs::File,
    io::{BufReader, Read},
};

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

fn read_csv<'a, R: Read + 'a>(input: R, name: &str, args: &Args) {
    let schema = Schema::new(&args.schema);

    // TODO: extract separator from file extension;
    let mut reader = BufReader::new(input);

    let mut parser = CSVParser::new(&mut reader, &schema, &args.separator);

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
        if !args.strict {
            println!("{}", obj);
        }
    }
}

fn main() {
    let args = Args::parse();

    for name in &args.filepaths {
        let path = match get_path(name) {
            Some(s) => s,
            None => return,
        };
        let file = File::open(path).expect("could not open csv");
        read_csv(file, name, &args);
    }
}
