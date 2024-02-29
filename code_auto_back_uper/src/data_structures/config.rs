use serde::{Deserialize, Serialize};
use std::collections::HashSet;

const DEFAULT_CHANGE_DETECTION_BUFFER: u64 = 5;
const DEFAULT_BACKUP_FREQUENCY: u64 = 5;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub watching_folders: HashSet<String>,
    pub backup_frequency: u64,
    pub change_detection_buffer: u64,
    pub ssh_private_key_path: String,
}

impl Config {
    pub fn new() -> Config {
        let default_ssh_path = match std::env::var("HOME") {
            Ok(home) => {
                format!("{}/.ssh/id_rsa", home)
            }
            Err(_) => "".to_string(),
        };

        Config {
            watching_folders: HashSet::new(),
            backup_frequency: DEFAULT_BACKUP_FREQUENCY,
            change_detection_buffer: DEFAULT_CHANGE_DETECTION_BUFFER,
            ssh_private_key_path: default_ssh_path,
        }
    }

    pub fn insert_watching_folder(&mut self, folder: String) {
        if self.watching_folders.contains(&folder) {
            println!("{} is already being watched", folder);
        }

        //But actually nothing will happen if you add an existing folder
        self.watching_folders.insert(folder);
    }

    pub fn remove_watching_folder(&mut self, folder: &str) {
        if !self.watching_folders.contains(folder) {
            println!("{} is not being watched", folder);
            return;
        }

        //But actually nothing will happen if you remove a non-existing folder
        self.watching_folders.remove(folder);
    }
    pub fn set_ssh_private_key_path(&mut self, path: String) {
        self.ssh_private_key_path = path;
    }
    pub fn update_encrypted_access_token(&mut self) {
        todo!("Implement this")
    }
}
