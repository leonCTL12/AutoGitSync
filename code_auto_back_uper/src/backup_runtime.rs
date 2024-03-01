use git2::Repository;

use crate::{
    backup_executor,
    utilities::{copy_dir_api_wrapper, git2_api_wrapper},
};

pub struct BackupRuntime {
    pub path: String,
    pub repo: Repository,
}

impl BackupRuntime {
    pub fn new(repo_path: &str) -> Result<BackupRuntime, git2::Error> {
        let temp_clone_path = match copy_dir_api_wrapper::copy_directory(repo_path) {
            Ok(path) => path,
            Err(e) => {
                return Err(git2::Error::from_str(&format!(
                    "Failed to clone the repository: {}",
                    e
                )));
            }
        };

        let repo = match Repository::open(&temp_clone_path) {
            Ok(repo) => repo,
            Err(e) => {
                return Err(e);
            }
        };

        Ok(BackupRuntime {
            path: temp_clone_path.to_string(),
            repo,
        })
    }

    pub fn perform_backup(&mut self) -> Result<(), git2::Error> {
        if !self.should_perform_backup()? {
            return Ok(());
        }

        //Keep the reference of the current branch
        let current_branch = git2_api_wrapper::get_current_branch_name(&self.repo)?;

        let backup_branch_name = backup_executor::get_back_up_branch_name(&current_branch);

        git2_api_wrapper::create_branch(&self.repo, &backup_branch_name)?;

        git2_api_wrapper::stash_all_changes(&mut self.repo)?;

        git2_api_wrapper::checkout_to_branch(&self.repo, &backup_branch_name)?;

        git2_api_wrapper::try_apply_stash(&mut self.repo)?;

        git2_api_wrapper::stage_all_changes(&self.repo)?;

        git2_api_wrapper::commit_all_changes(&mut self.repo)?;

        git2_api_wrapper::push_to_remote(&self.repo, &backup_branch_name)?;

        git2_api_wrapper::checkout_to_branch(&self.repo, &current_branch)?;

        git2_api_wrapper::try_apply_stash(&mut self.repo)?;

        git2_api_wrapper::delete_latest_stash(&mut self.repo)?;

        Ok(())
    }

    fn should_perform_backup(&mut self) -> Result<bool, git2::Error> {
        //Keep the reference of the current branch
        let current_branch = git2_api_wrapper::get_current_branch_name(&self.repo)?;

        let latest_branch_name = match git2_api_wrapper::get_latest_backup_branch_name(&self.repo) {
            Some(branch_name) => branch_name,
            None => {
                return Ok(true);
            }
        };

        git2_api_wrapper::stash_all_changes(&mut self.repo)?;

        git2_api_wrapper::checkout_to_branch(&self.repo, &latest_branch_name)?;

        if let Err(e) = git2_api_wrapper::try_apply_stash(&mut self.repo) {
            self.restore_repo_state(current_branch)?;
            if e.message() != "Conflict detected" {
                return Err(e);
            }
            return Ok(true);
        }

        if !git2_api_wrapper::has_local_change(&self.repo)? {
            self.restore_repo_state(current_branch)?;
            println!("No need to backup");
            return Ok(false);
        }

        self.restore_repo_state(current_branch)?;

        Ok(true)
    }

    //Maybe this is no longer needed, coz this is a cloned repository
    pub fn restore_repo_state(&mut self, current_branch: String) -> Result<(), git2::Error> {
        git2_api_wrapper::discard_local_change(&self.repo)?;
        git2_api_wrapper::checkout_to_branch(&self.repo, &current_branch)?;
        git2_api_wrapper::try_apply_stash(&mut self.repo)?;
        git2_api_wrapper::delete_latest_stash(&mut self.repo)?;
        Ok(())
    }
}
