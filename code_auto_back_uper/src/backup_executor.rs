use crate::utilities::file_system;
use git2::{Repository, Signature};
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

    let branch_name = get_back_up_branch_name();

    match create_backup_branch_if_not_exists(&repo, &branch_name) {
        Ok(_) => println!("Backup branch is created successfully"),
        Err(e) => {
            println!("Failed to create a backup branch: {}", e);
            return;
        }
    }
    match checkout_to_branch(&repo, &branch_name) {
        Ok(_) => println!("Checkout to the backup branch successfully"),
        Err(e) => {
            println!("Failed to checkout to the backup branch: {}", e)
        }
    }

    match commit_all_changes(&mut repo) {
        Ok(_) => println!("Backup is done successfully"),
        Err(e) => {
            println!("Failed to commit the changes: {}", e);
            return;
        }
    }

    //TODO: finally check back to the original branch
}

fn get_back_up_branch_name() -> String {
    let os = os_type().unwrap_or("Unknown_os".to_string());
    let host = hostname().unwrap_or("Unknown_host".to_string());
    let cpu_count = cpu_num().unwrap_or(0);

    format!("GitAutoBackup_{}_{}_{}", os, host, cpu_count)
}

fn create_backup_branch_if_not_exists(
    repo: &Repository,
    branch_name: &str,
) -> Result<(), git2::Error> {
    if repo
        .find_branch(branch_name, git2::BranchType::Local)
        .is_ok()
    {
        return Ok(());
    }

    let oid = repo.refname_to_id("HEAD")?;

    let commit = repo.find_commit(oid)?;

    repo.branch(branch_name, &commit, false)?;
    println!("Created a new branch: {}", branch_name);
    Ok(())
}

fn checkout_to_branch(repo: &Repository, branch_name: &str) -> Result<(), git2::Error> {
    let branch = repo.find_branch(branch_name, git2::BranchType::Local)?;
    let obj = branch.get().peel(git2::ObjectType::Commit)?;
    repo.checkout_tree(&obj, None)?;
    repo.set_head(&format!("refs/heads/{}", branch_name))?;
    Ok(())
}

fn commit_all_changes(repo: &mut Repository) -> Result<(), git2::Error> {
    let mut index = repo.index()?;

    //Check for changes
    let mut opts = git2::DiffOptions::new();
    let diff = repo.diff_index_to_workdir(None, Some(&mut opts))?;
    let has_changes = diff.deltas().count() > 0;
    if !has_changes {
        println!("No changes to commit");
        return Ok(());
    }

    //Stage all the changes
    index.add_all(["."].iter(), git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;

    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    let head = repo.head()?.peel_to_commit()?;

    let signature = Signature::now("Git Auto Sync Bot", "neverlostanyprogress@mail.com")?;
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
