use clap::Parser;
use serde_json::{Map, Value};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

mod cli;
mod models;

use crate::cli::Args;

fn clean_line(line: &str) -> String {
    let cleaned: String = line
        .chars()
        .filter(|&c| (c.is_ascii()) && c != '\n' && c != '\r' && c != '\\' && c != '"')
        .collect();
    cleaned.trim().to_owned()
}

fn read_line(reader: &mut BufReader<File>) -> Option<String> {
    let mut buf: String = "".to_string();
    let byte_count = match reader.read_line(&mut buf) {
        Ok(x) if x == 0 => return None,
        Ok(x) => x,
        Err(_) => return None,
    };
    Some(buf[0..byte_count].to_string())
}

fn get_path(name: &str) -> Option<PathBuf> {
    let file_path = Path::new(name);
    if file_path.is_absolute() {
        return Some(file_path.to_path_buf());
    }

    let cwd = std::env::current_dir().ok()?;
    Some(cwd.as_path().join(name))
}

fn read_file(path: &str, args: &Args) -> HashMap<String, f64> {
    let file_path = get_path(path).unwrap();
    let file = std::fs::File::open(file_path).unwrap();

    let mut reader = BufReader::new(file);

    let first_line = read_line(&mut reader).unwrap();

    let headers: Vec<String> = first_line
        .split(&args.separator)
        .map(|s| clean_line(s))
        .collect();

    let key_idx = headers
        .iter()
        .enumerate()
        .find(|(_, x)| args.keys.contains(*x))
        .map(|(i, _)| i)
        .expect("header key not found");

    let value_idxs: Vec<usize> = headers
        .iter()
        .enumerate()
        .filter(|(_, x)| args.values.contains(*x))
        .map(|(i, _)| i)
        .collect();

    let mut items: HashMap<String, f64> = HashMap::new();

    loop {
        let line = match read_line(&mut reader) {
            Some(s) => s,
            None => break,
        };

        let fields: Vec<&str> = line.split(&args.separator).collect();

        let k_value: String = clean_line(fields[key_idx]);
        let v_values: Vec<String> = value_idxs
            .iter()
            .map(|i| {
                fields[*i]
                    .chars()
                    .filter(|c| c.is_digit(10) || *c == '.' || *c == '-')
                    .collect()
            })
            .collect();

        let sum: f64 = v_values.iter().filter_map(|x| x.parse::<f64>().ok()).sum();
        if sum == 0.0 {
            continue;
        }

        let new_val = match items.get(&k_value) {
            Some(x) => x + sum,
            _ => sum,
        };

        items.insert(k_value, new_val);
    }
    items
}

fn merge(combined: &mut Map<String, Value>, new: HashMap<String, f64>) {
    for (key, value) in new {
        let new_val = match combined.get(&key) {
            Some(Value::Number(x)) => x.as_f64().unwrap() + value,
            _ => value,
        };

        let json_val = Value::from(new_val);
        combined.insert(key.clone(), json_val);
    }
}

fn main() {
    let args = Args::parse();

    let mut combined: Map<String, Value> = Map::new();

    for path in &args.filepaths {
        let map = read_file(path, &args);
        merge(&mut combined, map);
    }

    let json = serde_json::Value::Object(combined);
    println!("{}", serde_json::to_string_pretty(&json).unwrap());
}
