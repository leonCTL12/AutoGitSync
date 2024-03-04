use std::path::{self, Path};

use crate::temp_clone_repo::TempCloneRepo;
use crate::utilities::file_system;
use chrono::{DateTime, Utc};
use ignore::gitignore::Gitignore;

pub struct RepositoryInstance {
    path: String,
    last_update_time: Option<DateTime<Utc>>,
    dirty: bool,
    git_ignore: Gitignore,
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

        let git_ignore = match RepositoryInstance::create_gitignore(repo_path) {
            Ok(git_ignore) => git_ignore,
            Err(e) => return Err(e),
        };

        Ok(RepositoryInstance {
            path: repo_path.to_string(),
            last_update_time: None,
            dirty: false, //by default, it is not dirty
            git_ignore,
        })
    }

    fn create_gitignore(path: &str) -> Result<Gitignore, String> {
        let gitignore_path = Path::new(path).join(".gitignore");
        let git_ignore = Gitignore::new(gitignore_path);

        if git_ignore.1.is_some() {
            return Err(format!("Failed to create gitignore for {}", path));
        }

        let git_ignore = git_ignore.0;
        Ok(git_ignore)
    }

    pub fn try_perform_backup(&mut self) -> Result<(), git2::Error> {
        if !self.should_perform_backup() {
            println!("No need to perform backup for {}", self.path);
            return Ok(());
        }
        let mut temp_clone_repo = TempCloneRepo::new(&self.path)?;

        match temp_clone_repo.perform_backup() {
            Ok(_) => {
                println!("Backup done for {}", self.path);
                self.last_update_time = None;
                self.dirty = false;
                Ok(())
            }
            Err(e) => {
                println!("Failed to perform backup for {}: {}", self.path, e);
                Err(e)
            }
        }
    }

    pub fn handle_file_change(&mut self, path: &String, date_time: DateTime<Utc>) {
        let absolute_path = Path::new(path);
        let relative_path = absolute_path.strip_prefix(&self.path).unwrap();

        let is_ignored = self.git_ignore.matched(relative_path, false).is_ignore();

        if is_ignored {
            println!("{} is ignored", path);
            return;
        }

        self.dirty = true;
        self.last_update_time = Some(date_time);
        println!("{} is updated", relative_path.to_str().unwrap());
    }

    fn should_perform_backup(&self) -> bool {
        if !self.dirty {
            return false;
        }

        let config = crate::config_manager::read_config();

        match self.last_update_time {
            None => false,
            Some(last_update_time) => {
                let current_time = Utc::now();
                let duration = current_time
                    .signed_duration_since(last_update_time)
                    .num_seconds(); //this is second for testing purpose, it should be minutes instead
                duration >= config.change_detection_buffer as i64
            }
        }
    }
}
