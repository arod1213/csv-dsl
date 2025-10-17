use std::{collections::HashMap, fs::File, io::BufReader};

use serde::Deserialize;

use crate::read::get_path;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct FieldSpec {
    pub name: String,

    #[serde(default)]
    pub r#type: String,

    #[serde(default)]
    pub optional: bool,

    #[serde(default)]
    pub aliases: Vec<String>,
    // #[serde(default)]
    // normalize: Option<String>,
}

// TODO: optimize this to not have entries per alias
pub fn yaml_schema(path_str: &str) -> HashMap<String, FieldSpec> {
    let path = get_path(path_str).expect("schema could not be found");
    let file = File::open(path).expect("no schema found");
    let mut reader = BufReader::new(file);
    let schema = serde_yaml::from_reader::<_, Vec<FieldSpec>>(&mut reader).expect("invalid schema");
    let mut map: HashMap<String, FieldSpec> = HashMap::new();
    for spec in schema.iter() {
        for alias in spec.aliases.iter() {
            map.insert(alias.to_string(), spec.clone());
        }
    }
    map
}
