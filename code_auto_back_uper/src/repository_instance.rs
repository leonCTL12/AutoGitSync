use crate::backup_executor;
use crate::utilities::file_system;
use crate::utilities::git2_api_wrapper;
use chrono::{DateTime, Utc};

use git2::Repository;
pub struct RepositoryInstance {
    repo: Repository,
    pub path: String,
    last_update_time: Option<DateTime<Utc>>,
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
        Ok(RepositoryInstance {
            repo,
            path: repo_path.to_string(),
            last_update_time: None,
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

        git2_api_wrapper::try_apply_stash(&mut self.repo);

        git2_api_wrapper::stage_all_changes(&self.repo)?;

        git2_api_wrapper::commit_all_changes(&mut self.repo);

        git2_api_wrapper::push_to_remote(&self.repo, &backup_branch_name);

        git2_api_wrapper::checkout_to_branch(&self.repo, &current_branch);

        git2_api_wrapper::try_apply_stash(&mut self.repo);

        git2_api_wrapper::delete_latest_stash(&mut self.repo);

        self.last_update_time = Some(Utc::now());
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
        let config = crate::config_manager::read_config();
        match self.last_update_time {
            None => Ok(true),
            Some(last_update_time) => {
                let current_time = Utc::now();
                let duration = current_time.signed_duration_since(last_update_time);
                Ok(duration.num_minutes() >= config.change_detection_buffer as i64)
            }
        }
    }

    fn restore_repo_state(&mut self, current_branch: String) -> Result<(), git2::Error> {
        git2_api_wrapper::checkout_to_branch(&self.repo, &current_branch)?;
        git2_api_wrapper::try_apply_stash(&mut self.repo)?;
        git2_api_wrapper::delete_latest_stash(&mut self.repo)?;
        Ok(())
    }
}
