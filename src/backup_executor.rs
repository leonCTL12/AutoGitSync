use crate::file_change_watcher::FileChangeSignal;
use crate::repository_instance::RepositoryInstance;
use crate::{config_manager, file_change_watcher};
use chrono::Local;
use std::collections::{hash_map::Entry::Vacant, HashMap};
use std::sync::mpsc::{self, Receiver, Sender};
use sys_info::hostname;

pub struct BackupExecutor {
    //Has to be a hashset for easy removal
    map: HashMap<String, RepositoryInstance>,
}

impl BackupExecutor {
    pub fn new() -> BackupExecutor {
        BackupExecutor {
            map: HashMap::new(),
        }
    }

    pub fn start(&mut self) {
        let (tx, rx): (Sender<FileChangeSignal>, Receiver<FileChangeSignal>) = mpsc::channel();

        file_change_watcher::start(tx);
        loop {
            //continue to process messages from the channel until try_recv() returns an error, indicating that the channel is empty.
            while let Ok(signal) = rx.try_recv() {
                self.update_repo_instance_states(signal);
            }

            //To handle the disconnect case properly
            if let Err(e) = rx.try_recv() {
                match e {
                    std::sync::mpsc::TryRecvError::Empty => {}
                    std::sync::mpsc::TryRecvError::Disconnected => {
                        println!("File change event channel disconnected");
                        return;
                    }
                }
            }

            self.update_map();
            self.backup_check();

            let backup_frequency = if cfg!(debug_assertions) {
                5
            } else {
                println!(
                    "Sleeping for {} seconds",
                    config_manager::read_config().backup_frequency * 60
                );
                config_manager::read_config().backup_frequency * 60
            };

            std::thread::sleep(std::time::Duration::from_secs(backup_frequency));
        }
    }

    //Trade-off noted: this is not a pure function, yet it prevents cloning the map
    fn update_repo_instance_states(&mut self, signal: FileChangeSignal) {
        for (repo_path, repo) in &mut self.map {
            for file_path in &signal.paths {
                if file_path.starts_with(repo_path) {
                    repo.handle_file_change(file_path, signal.timestamp);
                    return;
                }
            }
        }
    }

    fn update_map(&mut self) {
        let config = config_manager::read_config();

        if config.watching_folders.is_empty() {
            println!(
                "No folder is being watched, please use the watch command to add a folder to watch"
            );
            return;
        }

        //TODO: one more case to handle: removing the folder from the watch list
        for folder in config.watching_folders {
            //Map.entry returns Vacant or Occupied
            if let Vacant(_) = self.map.entry(folder.clone()) {
                let repo_instance = match RepositoryInstance::new(&folder) {
                    Ok(repo) => repo,
                    Err(_) => {
                        println!("Error creating repository instance for {}", folder);
                        continue;
                    }
                };
                self.map.insert(folder, repo_instance);
            }
        }
    }
    //Trade-off noted: this is not a pure function, yet it prevents cloning the map
    fn backup_check(&mut self) {
        println!("Performing backup check");

        for repo_instance in &mut self.map.values_mut() {
            match repo_instance.try_perform_backup() {
                Ok(_) => {}
                Err(e) => {
                    println!("Failed to perform backup: {}", e);
                }
            }
        }
    }
}

pub fn get_back_up_branch_name(current_branch_name: &str) -> String {
    let host = hostname().unwrap_or("Unknown_host".to_string());
    let current_time = Local::now();

    format!(
        "backup/{}/{}_{}",
        host,
        current_branch_name,
        current_time.format("%Y-%m-%d_%H-%M-%S")
    )
}
