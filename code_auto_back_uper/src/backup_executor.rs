use crate::repository_instance::RepositoryInstance;
use crate::{config_manager, file_change_watcher};
use chrono::Local;
use std::collections::{hash_map::Entry::Vacant, HashMap};
use std::sync::mpsc::{self, Receiver, Sender};
use sys_info::hostname;

pub fn start() {
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    file_change_watcher::start(tx);
    let mut map: HashMap<String, RepositoryInstance> = HashMap::new();

    loop {
        match rx.try_recv() {
            //TODO:This should send a struct with path and timestamp instead of just path
            Ok(path) => {
                println!("Main thread received! File change event: {:?}", path);
            }
            Err(e) => match e {
                std::sync::mpsc::TryRecvError::Empty => {}
                std::sync::mpsc::TryRecvError::Disconnected => {
                    println!("File change event channel disconnected");
                    break;
                }
            },
        }

        //Check everyloop so that it reacts to the new setting
        let config = config_manager::read_config();
        backup_check(&mut map);
        std::thread::sleep(std::time::Duration::from_secs(config.backup_frequency));
    }
}

fn backup_check(map: &mut HashMap<String, RepositoryInstance>) {
    println!("Performing backup");
    let config = config_manager::read_config();

    if config.watching_folders.is_empty() {
        println!(
            "No folder is being watched, please use the watch command to add a folder to watch"
        );
        return;
    }

    for folder in config.watching_folders {
        //Map.entry returns Vacant or Occupied
        if let Vacant(_) = map.entry(folder.clone()) {
            let repo_instance = match RepositoryInstance::new(&folder) {
                Ok(repo) => repo,
                Err(_) => {
                    println!("Error creating repository instance for {}", folder);
                    continue;
                }
            };
            map.insert(folder, repo_instance);
        }
    }

    for repo_instance in map.values_mut() {
        repo_instance.try_perform_backup();
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
