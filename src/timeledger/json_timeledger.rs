use serde::Deserialize;
use serde_json::Result;
use std::collections::HashMap;

type JsonTask = Vec<String>;

type JsonDay = Vec<JsonTask>;

type JsonDays = HashMap<String, JsonDay>;

#[derive(Debug, Deserialize)]
pub struct JsonTimeledger {
    pub timeledger: Vec<JsonDays>
}

impl JsonTimeledger {
    pub fn new(json: &String) -> Result<JsonTimeledger> {
        serde_json::from_str(json)
    }
}
