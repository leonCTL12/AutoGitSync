use crate::backup_runtime::BackupRuntime;
use crate::utilities::file_system;
use chrono::{DateTime, Utc};

pub struct RepositoryInstance {
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

        Ok(RepositoryInstance {
            path: repo_path.to_string(),
            last_update_time: None,
        })
    }

    pub fn try_perform_backup(&mut self) -> Result<(), git2::Error> {
        if !self.is_time_to_backup() {
            return Ok(());
        }
        let mut backup_runtime = BackupRuntime::new(&self.path)?;

        match backup_runtime.perform_backup() {
            Ok(_) => {
                println!("Backup done for {}", self.path);
                self.last_update_time = Some(Utc::now());
                Ok(())
            }
            Err(e) => {
                println!("Failed to perform backup for {}: {}", self.path, e);
                //TODO: restore the repo state properly
                Err(e)
            }
        }
    }

    fn is_time_to_backup(&self) -> bool {
        let config = crate::config_manager::read_config();

        match self.last_update_time {
            None => true,
            Some(last_update_time) => {
                let current_time = Utc::now();
                let duration = current_time.signed_duration_since(last_update_time);
                duration.num_minutes() >= config.change_detection_buffer as i64
            }
        }
    }
}
