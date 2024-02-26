use serde::{Deserialize, Serialize};
use std::collections::HashSet;

const DEFAULT_CHANGE_DETECTION_BUFFER: u64 = 5;
const DEFAULT_BACKUP_FREQUENCY: u64 = 5;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub watching_folders: HashSet<String>,
    pub backup_frequency: u64,
    pub change_detection_buffer: u64,
}

impl Config {
    pub fn new() -> Config {
        Config {
            watching_folders: HashSet::new(),
            backup_frequency: DEFAULT_BACKUP_FREQUENCY,
            change_detection_buffer: DEFAULT_CHANGE_DETECTION_BUFFER,
        }
    }

    pub fn insert_watching_folder(&mut self, folder: String) {
        self.watching_folders.insert(folder);
    }

    pub fn update_encrypted_access_token(&mut self) {
        todo!("Implement this")
    }
}
