use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize)]
pub struct Config {
    watching_folders: HashSet<String>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            watching_folders: HashSet::new(),
        }
    }

    pub fn insert_watching_folder(&mut self, folder: String) {
        self.watching_folders.insert(folder);
    }
}
