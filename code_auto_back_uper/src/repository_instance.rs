use crate::backup_executor;
use crate::utilities::file_system;
use crate::utilities::git2_api_wrapper;
use git2::Repository;
pub struct RepositoryInstance {
    repo: Repository,
}

impl RepositoryInstance {
    pub fn new(repo_path: &str) -> Result<RepositoryInstance, String> {
        let repo_path: &str = repo_path;
        if !file_system::is_git_repository(repo_path) {
            return Err(format!(
                "{} is not a git repository, it will be ignored",
                repo_path
            ));
        }

        let repo = match Repository::open(repo_path) {
            Ok(repo) => repo,
            Err(e) => {
                return Err(format!("Failed to open the repository: {}", e));
            }
        };
        Ok(RepositoryInstance { repo })
    }

    pub fn perform_backup(&mut self) {
        match git2_api_wrapper::has_local_change(&self.repo) {
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
        let current_branch = match git2_api_wrapper::get_current_branch_name(&self.repo) {
            Ok(branch) => branch,
            Err(e) => {
                println!("Failed to get the current branch: {}", e);
                return;
            }
        };

        let backup_branch_name = backup_executor::get_back_up_branch_name(&current_branch);

        match git2_api_wrapper::create_branch(&self.repo, &backup_branch_name) {
            Ok(_) => println!("Backup branch is created successfully"),
            Err(e) => {
                println!("Failed to create a backup branch: {}", e);
                return;
            }
        }

        match git2_api_wrapper::stash_all_changes(&mut self.repo) {
            Ok(_) => println!("Stashed all the changes successfully"),
            Err(e) => {
                println!("Failed to stash the changes: {}", e);
                return;
            }
        }

        match git2_api_wrapper::checkout_to_branch(&self.repo, &backup_branch_name) {
            Ok(_) => println!("Checkout to the backup branch successfully"),
            Err(e) => {
                println!("Failed to checkout to the backup branch: {}", e)
            }
        }

        match git2_api_wrapper::apply_stash(&mut self.repo, false) {
            Ok(_) => println!("Applied the stash successfully"),
            Err(e) => {
                println!("Failed to apply the stash: {}", e);
                return;
            }
        }

        match git2_api_wrapper::stage_all_changes(&self.repo) {
            Ok(_) => println!("Staged all the changes successfully"),
            Err(e) => {
                println!("Failed to stage the changes: {}", e);
                return;
            }
        }

        match git2_api_wrapper::commit_all_changes(&mut self.repo) {
            Ok(_) => println!("Committed the changes successfully"),
            Err(e) => {
                println!("Failed to commit the changes: {}", e);
                return;
            }
        }

        match git2_api_wrapper::push_to_remote(&self.repo, &backup_branch_name) {
            Ok(_) => println!("Pushed the changes to the remote successfully"),
            Err(e) => {
                println!("Failed to push the changes to the remote: {}", e);
                return;
            }
        }

        match git2_api_wrapper::checkout_to_branch(&self.repo, &current_branch) {
            Ok(_) => println!("Checkout back to the original branch successfully"),
            Err(e) => {
                println!("Failed to checkout back to the original branch: {}", e);
                return;
            }
        }

        match git2_api_wrapper::apply_stash(&mut self.repo, true) {
            Ok(_) => println!("Applied the stash successfully"),
            Err(e) => {
                println!("Failed to apply the stash: {}", e);
                return;
            }
        }

        println!("Backup is done");
    }
}
