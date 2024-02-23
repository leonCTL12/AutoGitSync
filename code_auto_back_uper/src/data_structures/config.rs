use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub watching_folders: HashSet<String>,
    pub encrypted_access_token: String,
}

impl Config {
    pub fn new() -> Config {
        Config {
            watching_folders: HashSet::new(),
            encrypted_access_token: String::new(),
        }
    }

    pub fn insert_watching_folder(&mut self, folder: String) {
        self.watching_folders.insert(folder);
    }

    pub fn update_encrypted_access_token(&mut self, token: String) {
        self.encrypted_access_token = token;
    }
}
