use crate::utilities::file_system;
use git2::Repository;
use sys_info::{cpu_num, hostname, mem_info, os_type};

use crate::config_manager;

pub fn start() {
    loop {
        backup_check();
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}

fn backup_check() {
    println!("Performing backup");
    let config = config_manager::read_config();

    if config.watching_folders.is_empty() {
        println!(
            "No folder is being watched, please use the watch command to add a folder to watch"
        );
        return;
    }

    for folder in config.watching_folders {
        perform_backup(&folder);
    }
}

fn perform_backup(repo_path: &str) {
    if !file_system::is_git_repository(repo_path) {
        println!("{} is not a git repository, it will be ignored", repo_path);
        return;
    }

    let repo = match Repository::open(repo_path) {
        Ok(repo) => repo,
        Err(e) => {
            println!("Failed to open the repository: {}", e);
            return;
        }
    };

    let branch_name = get_back_up_branch_name();

    create_backup_branch_if_not_exists(&repo, &branch_name);
}

fn get_back_up_branch_name() -> String {
    let os = os_type().unwrap_or("Unknown_os".to_string());
    let host = hostname().unwrap_or("Unknown_host".to_string());
    let cpu_count = cpu_num().unwrap_or(0);

    format!("GitAutoBackup_{}_{}_{}", os, host, cpu_count)
}

fn create_backup_branch_if_not_exists(repo: &Repository, branch_name: &str) {
    if !repo
        .find_branch(&branch_name, git2::BranchType::Local)
        .is_ok()
    {
        let oid = match repo.refname_to_id("HEAD") {
            Ok(oid) => oid,
            Err(e) => {
                println!("Failed to get the oid of HEAD: {}", e);
                return;
            }
        };

        let commit = match repo.find_commit(oid) {
            Ok(commit) => commit,
            Err(e) => {
                println!("Failed to find the commit of HEAD: {}", e);
                return;
            }
        };

        match repo.branch(&branch_name, &commit, false) {
            Ok(_) => println!("Branch {} is created", branch_name),
            Err(e) => println!("Failed to create branch {}: {}", branch_name, e),
        }
    }
}
