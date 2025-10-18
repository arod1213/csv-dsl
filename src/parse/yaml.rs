use std::{collections::HashMap, fs::File, io::BufReader};

use serde::Deserialize;

use crate::read::get_path;

#[derive(Debug, Deserialize, Clone)]
pub enum DataType {
    Float,
    Int,
    Uint,
    String,
    Bool,
    Date,
    Country,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct FieldSpec {
    pub name: String,

    pub r#type: DataType,

    #[serde(default)]
    pub optional: bool,

    #[serde(default)]
    pub aliases: Vec<String>,
    // #[serde(default)]
    // normalize: Option<String>,
}

pub struct Schema {
    pub specs: Vec<FieldSpec>,
    pub alias_to_name: HashMap<String, FieldSpec>,
}

impl Schema {
    pub fn new(path_str: &str) -> Self {
        let path = get_path(path_str).expect("schema could not be found");
        let file = File::open(path).expect("no schema found");
        let mut reader = BufReader::new(file);
        let schema =
            serde_yaml::from_reader::<_, Vec<FieldSpec>>(&mut reader).expect("invalid schema");
        let mut map: HashMap<String, FieldSpec> = HashMap::new();
        for spec in schema.iter() {
            for alias in spec.aliases.iter() {
                map.insert(alias.to_string(), spec.clone());
            }
        }
        Schema {
            specs: schema,
            alias_to_name: map,
        }
    }
}
