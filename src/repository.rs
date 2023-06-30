use std::fs;
use std::error::Error;
use serde::{Serialize, Deserialize};
use crate::data::MatchUp;

pub struct MatchUpRepository {
    match_up_container: MatchUpContainer,
}

impl MatchUpRepository {
    pub fn new() -> Self {
        let serialized = match fs::read_to_string("data.json") {
            Ok(serialized) => serialized,
            Err(_) => {
                let match_up_container = MatchUpContainer {
                    match_ups: Vec::new(),
                };
                let serialized = serde_json::to_string(&match_up_container).unwrap();
                fs::File::create("data.json").unwrap();
                fs::write("data.json", &serialized).unwrap();
                serialized
            }
        };
        let match_up_container: MatchUpContainer = serde_json::from_str(&serialized).unwrap();

        MatchUpRepository {
            match_up_container,
        }
    }

    pub fn add_match_up(&mut self, match_up: MatchUp) -> Result<(), Box<dyn Error>> {
        self.match_up_container.match_ups.push(match_up);
        self.save();
        Ok(())
    }

    pub fn get_match_ups(&mut self) -> &mut Vec<MatchUp> {
        &mut self.match_up_container.match_ups
    }

    pub fn save(&self) {
        let serialized = serde_json::to_string(&self.match_up_container).unwrap();
        fs::File::create("data.json").unwrap();
        fs::write("data.json", serialized).unwrap();
    }

    pub fn delete(&mut self, index: usize) {
        self.match_up_container.match_ups.remove(index);
        self.save();
    }
}

#[derive(Serialize, Deserialize)]
struct MatchUpContainer {
    match_ups: Vec<MatchUp>,
}
