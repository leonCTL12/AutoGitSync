use crate::config_manager;
use crate::repository_instance::RepositoryInstance;
use chrono::Local;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::{
    collections::{hash_map::Entry::Vacant, HashMap},
    path::Path,
};
use sys_info::hostname;

fn test_notify() -> notify::Result<()> {
    // Automatically select the best implementation for your platform.
    let mut watcher = notify::recommended_watcher(|res| match res {
        Ok(event) => println!("event: {:?}", event),
        Err(e) => println!("watch error: {:?}", e),
    })?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    let config = config_manager::read_config();
    for folder in config.watching_folders {
        watcher.watch(Path::new(&folder), RecursiveMode::Recursive)?;
    }
    Ok(())
}

pub fn start() {
    test_notify();

    // let mut map: HashMap<String, RepositoryInstance> = HashMap::new();

    // loop {
    //     //Check everyloop so that it reacts to the new setting
    //     let config = config_manager::read_config();
    //     backup_check(&mut map);
    //     std::thread::sleep(std::time::Duration::from_secs(config.backup_frequency * 60));
    // }
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
        match repo_instance.perform_backup() {
            Ok(_) => println!("Backup check done for {}", repo_instance.path),
            Err(e) => println!("Backup failed: {}", e),
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
