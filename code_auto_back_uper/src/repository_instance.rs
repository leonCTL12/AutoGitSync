use crate::backup_executor;
use crate::utilities::file_system;
use crate::utilities::git2_api_wrapper;
use chrono::{DateTime, Utc};

use git2::Repository;
pub struct RepositoryInstance {
    repo: Repository,
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

        git2_api_wrapper::apply_stash(&mut self.repo, false);

        git2_api_wrapper::stage_all_changes(&self.repo)?;

        git2_api_wrapper::commit_all_changes(&mut self.repo);

        git2_api_wrapper::push_to_remote(&self.repo, &backup_branch_name);

        git2_api_wrapper::checkout_to_branch(&self.repo, &current_branch);

        git2_api_wrapper::apply_stash(&mut self.repo, true);

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

        //TODO: Consider abort when there is a conflict and return true (coz that mean there is something that needed to be backup)
        git2_api_wrapper::apply_stash(&mut self.repo, false)?;

        let has_local_change = git2_api_wrapper::has_local_change(&self.repo)?;
        git2_api_wrapper::checkout_to_branch(&self.repo, &current_branch)?;
        git2_api_wrapper::apply_stash(&mut self.repo, true)?;

        if !has_local_change {
            println!("No need to backup");
            return Ok(false);
        }
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
}
