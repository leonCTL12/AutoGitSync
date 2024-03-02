use crate::temp_clone_repo::TempCloneRepo;
use crate::utilities::file_system;
use chrono::{DateTime, Utc};

pub struct RepositoryInstance {
    path: String,
    last_update_time: Option<DateTime<Utc>>,
    dirty: bool,
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
            dirty: false, //by default, it is not dirty
        })
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
                self.last_update_time = Some(Utc::now());
                self.dirty = false;
                Ok(())
            }
            Err(e) => {
                println!("Failed to perform backup for {}: {}", self.path, e);
                Err(e)
            }
        }
    }

    pub fn set_dirty_flag(&mut self, date_time: DateTime<Utc>) {
        self.dirty = true;
        self.last_update_time = Some(date_time);
    }

    fn should_perform_backup(&self) -> bool {
        if !self.dirty {
            return false;
        }

        // let config = crate::config_manager::read_config();

        // match self.last_update_time {
        //     None => true,
        //     Some(last_update_time) => {
        //         let current_time = Utc::now();
        //         let duration = current_time.signed_duration_since(last_update_time);
        //         duration.num_minutes() >= config.change_detection_buffer as i64
        //     }
        // }

        return true;
    }
}
