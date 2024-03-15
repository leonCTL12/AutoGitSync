use crate::config_manager;
use crate::utilities::file_system;
use crate::{gitignore_wrapper::GitIgnoreWrapper, temp_clone_repo::TempCloneRepo};
use chrono::{DateTime, Utc};
use std::path::Path;

pub struct RepositoryInstance {
    repo_path: String,
    last_update_time: Option<DateTime<Utc>>,
    dirty: bool,
    git_ignore: GitIgnoreWrapper,
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

        let git_ignore = GitIgnoreWrapper::new(Path::new(repo_path).to_path_buf());

        Ok(RepositoryInstance {
            repo_path: repo_path.to_string(),
            last_update_time: None,
            dirty: false, //by default, it is not dirty
            git_ignore,
        })
    }

    pub fn try_perform_backup(&mut self) -> Result<(), git2::Error> {
        if !self.should_perform_backup() {
            println!("No need to perform backup for {}", self.repo_path);
            return Ok(());
        }
        let mut temp_clone_repo = TempCloneRepo::new(&self.repo_path)?;

        let result = temp_clone_repo.perform_backup();

        match temp_clone_repo.clean_temp_clone_folder() {
            Ok(_) => {}
            Err(e) => {
                println!("Failed to delete the temp clone repo: {}", e);
            }
        }

        match result {
            Ok(_) => {
                println!("Backup done for {}", self.repo_path);
                self.last_update_time = None;
                self.dirty = false;
                Ok(())
            }
            Err(e) => {
                println!("Failed to perform backup for {}: {}", self.repo_path, e);
                Err(e)
            }
        }
    }

    pub fn handle_file_change(&mut self, path: &String, date_time: DateTime<Utc>) {
        let absolute_path = Path::new(path);
        let is_ignored = self.git_ignore.query(absolute_path);

        if is_ignored {
            return;
        }

        println!("{} is changed", path);
        self.dirty = true;
        self.last_update_time = Some(date_time);
    }

    fn should_perform_backup(&self) -> bool {
        if !self.dirty {
            println!("{} is not dirty", self.repo_path);
            return false;
        }

        let config = config_manager::read_config();

        match self.last_update_time {
            None => false,
            Some(last_update_time) => {
                let current_time = Utc::now();

                let duration = current_time
                    .signed_duration_since(last_update_time)
                    .num_seconds();

                let change_detection_buffer = if cfg!(debug_assertions) {
                    5
                } else {
                    config.change_detection_buffer * 60
                } as i64;
                println!("Duration = {}", duration);
                println!("Change detection buffer = {}", change_detection_buffer);
                duration >= change_detection_buffer
            }
        }
    }
}
