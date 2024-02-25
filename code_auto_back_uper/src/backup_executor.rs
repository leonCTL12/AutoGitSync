use crate::config_manager;
use crate::repository_instance::RepositoryInstance;
use chrono::Local;
use std::collections::HashMap;
use sys_info::hostname;

pub fn start() {
    let mut map: HashMap<String, RepositoryInstance> = HashMap::new();

    loop {
        backup_check(&mut map);
        std::thread::sleep(std::time::Duration::from_secs(5));
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
        if !map.contains_key(&folder) {
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
        repo_instance.perform_backup();
    }
}

pub fn get_back_up_branch_name(current_branch_name: &str) -> String {
    let host = hostname().unwrap_or("Unknown_host".to_string());
    let current_time = Local::now();

    format!(
        "backup/{}_{}_{}",
        current_branch_name,
        host,
        current_time.format("%Y-%m-%d_%H-%M-%S")
    )
}
