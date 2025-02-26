use crate::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Rabit {
    pub name: String,
    pub init_date: String,
}

impl Rabit {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            init_date: "".to_string(),
        }
    }
}
