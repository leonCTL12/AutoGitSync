use crate::utilities::file_system;
use chrono::Local;
use git2::{Cred, PushOptions, RemoteCallbacks, Repository, Signature};
use sys_info::{cpu_num, hostname, os_type};

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

    let mut repo = match Repository::open(repo_path) {
        Ok(repo) => repo,
        Err(e) => {
            println!("Failed to open the repository: {}", e);
            return;
        }
    };

    match has_local_change(&repo) {
        Ok(false) => {
            println!("There are no local changes, no need to backup");
            return;
        }
        Ok(true) => println!("There are local changes, need to backup"),
        Err(e) => {
            println!("Failed to check local changes: {}", e);
            return;
        }
    }

    //Keep the reference of the current branch
    let current_branch = match get_current_branch_name(&repo) {
        Ok(branch) => branch,
        Err(e) => {
            println!("Failed to get the current branch: {}", e);
            return;
        }
    };

    let backup_branch_name = get_back_up_branch_name(&current_branch);

    match create_backup_branch(&repo, &backup_branch_name) {
        Ok(_) => println!("Backup branch is created successfully"),
        Err(e) => {
            println!("Failed to create a backup branch: {}", e);
            return;
        }
    }

    match stash_all_changes(&mut repo) {
        Ok(_) => println!("Stashed all the changes successfully"),
        Err(e) => {
            println!("Failed to stash the changes: {}", e);
            return;
        }
    }

    match checkout_to_branch(&repo, &backup_branch_name) {
        Ok(_) => println!("Checkout to the backup branch successfully"),
        Err(e) => {
            println!("Failed to checkout to the backup branch: {}", e)
        }
    }

    match apply_stash(&mut repo, false) {
        Ok(_) => println!("Applied the stash successfully"),
        Err(e) => {
            println!("Failed to apply the stash: {}", e);
            return;
        }
    }

    match stage_all_changes(&repo) {
        Ok(_) => println!("Staged all the changes successfully"),
        Err(e) => {
            println!("Failed to stage the changes: {}", e);
            return;
        }
    }

    match commit_all_changes(&mut repo) {
        Ok(_) => println!("Committed the changes successfully"),
        Err(e) => {
            println!("Failed to commit the changes: {}", e);
            return;
        }
    }

    match push_to_remote(&repo, &backup_branch_name) {
        Ok(_) => println!("Pushed the changes to the remote successfully"),
        Err(e) => {
            println!("Failed to push the changes to the remote: {}", e);
            return;
        }
    }

    match checkout_to_branch(&repo, &current_branch) {
        Ok(_) => println!("Checkout back to the original branch successfully"),
        Err(e) => {
            println!("Failed to checkout back to the original branch: {}", e);
            return;
        }
    }

    match apply_stash(&mut repo, true) {
        Ok(_) => println!("Applied the stash successfully"),
        Err(e) => {
            println!("Failed to apply the stash: {}", e);
            return;
        }
    }

    println!("Backup is done");
}

fn get_current_branch_name(repo: &Repository) -> Result<String, git2::Error> {
    let head = repo.head()?;
    let branch = head.shorthand().unwrap_or("Unknown_branch");
    Ok(branch.to_string())
}

fn get_back_up_branch_name(current_branch_name: &str) -> String {
    let host = hostname().unwrap_or("Unknown_host".to_string());
    let current_time = Local::now();

    format!(
        "backup/{}_{}_{}",
        current_branch_name,
        host,
        current_time.format("%Y-%m-%d_%H-%M-%S")
    )
}

fn create_backup_branch(repo: &Repository, branch_name: &str) -> Result<(), git2::Error> {
    let oid = repo.refname_to_id("HEAD")?;

    let commit = repo.find_commit(oid)?;

    repo.branch(branch_name, &commit, false)?;
    println!("Created a new branch: {}", branch_name);
    Ok(())
}

fn stash_all_changes(repo: &mut Repository) -> Result<(), git2::Error> {
    let signautre = repo.signature()?;
    let message = "Git auto sync stash";
    let flags = git2::StashFlags::DEFAULT;
    repo.stash_save(&signautre, message, Some(flags))?;
    Ok(())
}

fn checkout_to_branch(repo: &Repository, branch_name: &str) -> Result<(), git2::Error> {
    let branch = repo.find_branch(branch_name, git2::BranchType::Local)?;
    let obj = branch.get().peel(git2::ObjectType::Commit)?;
    repo.checkout_tree(&obj, None)?;
    repo.set_head(&format!("refs/heads/{}", branch_name))?;
    Ok(())
}

fn apply_stash(repo: &mut Repository, delete_after_apply: bool) -> Result<(), git2::Error> {
    let stash_index = 0; //The latest stash
    let mut options = git2::StashApplyOptions::default();
    repo.stash_apply(stash_index, Some(&mut options))?;
    if delete_after_apply {
        repo.stash_drop(stash_index)?;
    }

    Ok(())
}

fn has_local_change(repo: &Repository) -> Result<bool, git2::Error> {
    //Check for changes
    let mut opts = git2::DiffOptions::new();
    let diff = repo.diff_index_to_workdir(None, Some(&mut opts))?;
    Ok(diff.deltas().count() > 0)
}

fn stage_all_changes(repo: &Repository) -> Result<(), git2::Error> {
    let mut index = repo.index()?;

    //Stage all the changes
    index.add_all(["."].iter(), git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;

    Ok(())
}

fn commit_all_changes(repo: &mut Repository) -> Result<(), git2::Error> {
    let mut index = repo.index()?;

    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    let head = repo.head()?.peel_to_commit()?;

    let signature = Signature::now("Auto Git Bot", "makeup@gmail.com")?;
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        "Auto backup",
        &tree,
        &[&head],
    )?;

    Ok(())
}

fn push_to_remote(repo: &Repository, branch_name: &str) -> Result<(), git2::Error> {
    let mut remote = repo.find_remote("origin")?;

    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::ssh_key(
            username_from_url.unwrap(),
            None,
            std::path::Path::new(&format!("{}/.ssh/id_rsa", std::env::var("HOME").unwrap())),
            None,
        )
    });

    let mut push_options = PushOptions::new();
    push_options.remote_callbacks(callbacks);

    remote.push(
        &[&format!("refs/heads/{}", branch_name)],
        Some(&mut push_options),
    )?;

    Ok(())
}
