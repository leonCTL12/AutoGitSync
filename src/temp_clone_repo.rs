use git2::Repository;

use crate::{
    backup_executor,
    utilities::{
        copy_dir_api_wrapper,
        git2_api_wrapper::{self, AuthType},
    },
};

pub struct TempCloneRepo {
    pub repo: Repository,
    pub path: String,
    pub auth_type: AuthType,
}

impl TempCloneRepo {
    pub fn new(repo_path: &str) -> Result<TempCloneRepo, git2::Error> {
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

        let auth_type = match get_auth_type(&repo) {
            Ok(auth_type) => auth_type,
            Err(e) => {
                return Err(git2::Error::from_str(&format!(
                    "Failed to get the auth type: {}",
                    e
                )));
            }
        };

        println!("{} is a {:?} repo", temp_clone_path, auth_type);

        Ok(TempCloneRepo {
            repo,
            path: temp_clone_path,
            auth_type,
        })
    }

    pub fn perform_backup(&mut self) -> Result<(), git2::Error> {
        //Keep the reference of the current branch
        let has_local_change = git2_api_wrapper::has_local_changes(&self.repo)?;

        if !has_local_change {
            println!("No local changes, no need to perform backup");
            return Ok(());
        }

        let current_branch = git2_api_wrapper::get_current_branch_name(&self.repo)?;

        let backup_branch_name = backup_executor::get_back_up_branch_name(&current_branch);

        git2_api_wrapper::create_branch(&self.repo, &backup_branch_name)?;

        git2_api_wrapper::stash_all_changes(&mut self.repo)?;

        git2_api_wrapper::checkout_to_branch(&self.repo, &backup_branch_name)?;

        git2_api_wrapper::try_apply_stash(&mut self.repo)?;

        git2_api_wrapper::stage_all_changes(&self.repo)?;

        git2_api_wrapper::commit_all_changes(&mut self.repo)?;

        git2_api_wrapper::push_to_remote(&self.repo, &backup_branch_name, &self.auth_type)?;

        Ok(())
    }

    pub fn clean_temp_clone_folder(self) -> std::io::Result<()> {
        //this line of code is necessary
        //Repository::open will lock the clone temp folder
        //In windowsos, it will cause the remove_dir_all to fail
        //But it is fine on macos
        std::mem::drop(self.repo);
        println!("About to remove {}", &self.path);
        std::fs::remove_dir_all(&self.path)
    }
}

fn get_auth_type(repo: &Repository) -> Result<AuthType, String> {
    // Get all remotes
    let remotes = match repo.remotes() {
        Ok(remotes) => remotes,
        Err(e) => panic!("failed to get remotes: {}", e),
    };

    for remote in remotes.iter() {
        let remote_name = match remote {
            Some(remote) => remote,
            None => continue,
        };

        let url = match repo.find_remote(remote_name) {
            Ok(remote) => match remote.url() {
                Some(url) => url.to_string(),
                None => continue,
            },
            Err(_) => continue,
        };

        if url.starts_with("https") {
            return Ok(AuthType::Pat);
        } else if url.starts_with("git@") {
            return Ok(AuthType::Ssh);
        }
    }

    Err("Undefined Auth type".to_string())
}
