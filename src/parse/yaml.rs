use std::{collections::HashMap, fmt::format, fs::File, io::BufReader};

use serde::Deserialize;
use serde_json::Value;

use crate::{parse::validate::validate_field, read::get_path};

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
    default: Option<String>,
    pub default_value: Option<Value>,

    #[serde(default)]
    pub optional: bool,

    #[serde(default)]
    pub aliases: Vec<String>,
    // #[serde(default)]
    // normalize: Option<String>,
}
impl FieldSpec {
    pub fn init(&mut self) {
        let Some(def) = &self.default else {
            return;
        };
        self.default_value = match self.get_default(def) {
            Some(x) => Some(x),
            None => {
                let msg = format!("invalid default {} expected type {:?}", def, self.r#type);
                panic!("invalid default {} expected type {:?}", def, self.r#type);
            }
        };
    }

    fn get_default(&self, default: &str) -> Option<Value> {
        validate_field(default, self)
    }
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
        let mut schema =
            serde_yaml::from_reader::<_, Vec<FieldSpec>>(&mut reader).expect("invalid schema");
        let mut map: HashMap<String, FieldSpec> = HashMap::new();
        for spec in schema.iter_mut() {
            spec.init();
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
