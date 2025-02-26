use crate::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    pub rabits: Vec<Rabit>,
    pub tracks: HashMap<String, Vec<Rabit>>,
}

impl Data {
    pub fn default() -> Self {
        Self {
            rabits: vec![],
            tracks: HashMap::new(),
        }
    }

    pub fn track(&mut self, new_rabit: Rabit) {
        let mut has_rabit = false;
        for rabit in &self.rabits {
            if rabit.name == new_rabit.name {
                has_rabit = true;
                break;
            }
        }

        if !has_rabit {
            // !TODO prompt user if they want to create or not
            self.rabits.push(new_rabit);
        }
    }
}
