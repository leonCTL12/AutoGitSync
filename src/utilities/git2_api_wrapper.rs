use git2::{BranchType, Cred, PushOptions, RemoteCallbacks, Repository, ResetType, Signature};

use crate::utilities::secret_manager;

#[derive(Debug)]
pub enum AuthType {
    Ssh,
    Pat,
}

pub fn get_current_branch_name(repo: &Repository) -> Result<String, git2::Error> {
    let head = repo.head()?;
    let branch = head.shorthand().unwrap_or("Unknown_branch");
    Ok(branch.to_string())
}

pub fn create_branch(repo: &Repository, branch_name: &str) -> Result<(), git2::Error> {
    let oid = repo.refname_to_id("HEAD")?;

    let commit = repo.find_commit(oid)?;

    repo.branch(branch_name, &commit, false)?;
    println!("Created a new branch: {}", branch_name);
    Ok(())
}

pub fn stash_all_changes(repo: &mut Repository) -> Result<(), git2::Error> {
    let signautre = repo.signature()?;
    let message = "Git auto sync stash";
    let flags = git2::StashFlags::DEFAULT;
    repo.stash_save(&signautre, message, Some(flags))?;
    Ok(())
}

pub fn checkout_to_branch(repo: &Repository, branch_name: &str) -> Result<(), git2::Error> {
    let branch = repo.find_branch(branch_name, BranchType::Local)?;
    let obj = branch.get().peel(git2::ObjectType::Commit)?;
    repo.checkout_tree(&obj, None)?;
    repo.set_head(&format!("refs/heads/{}", branch_name))?;
    Ok(())
}

//This function will try to apply stash and abort if there is any conflict
pub fn try_apply_stash(repo: &mut Repository) -> Result<(), git2::Error> {
    let stash_index = 0; //The latest stash
    let mut options = git2::StashApplyOptions::default();
    repo.stash_apply(stash_index, Some(&mut options))?;

    let conflicts = repo.index()?.has_conflicts();
    if conflicts {
        println!("Conflict detected, aborting stash apply");
        let head = repo.head()?.peel_to_commit()?;
        repo.reset(&head.into_object(), ResetType::Hard, None)?;
        return Err(git2::Error::from_str("Conflict detected"));
    }

    Ok(())
}

pub fn stage_all_changes(repo: &Repository) -> Result<(), git2::Error> {
    let mut index = repo.index()?;

    //Stage all the changes
    index.add_all(["."].iter(), git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;

    Ok(())
}

pub fn commit_all_changes(repo: &mut Repository) -> Result<(), git2::Error> {
    let mut index = repo.index()?;

    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    let head = repo.head()?.peel_to_commit()?;

    let signature = Signature::now("Auto Git Bot", "makeup@gmail.com")?;
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

pub fn push_to_remote(
    repo: &Repository,
    branch_name: &str,
    auth_type: &AuthType,
) -> Result<(), git2::Error> {
    let mut remote = repo.find_remote("origin")?;

    let mut callbacks = RemoteCallbacks::new();

    match auth_type {
        AuthType::Ssh => {
            let ssh_private_key_path = match secret_manager::get_ssh_key_path() {
                Ok(path) => path,
                Err(_) => {
                    println!("Failed to get the ssh private key path");
                    return Err(git2::Error::from_str(
                        "Failed to get the ssh private key path",
                    ));
                }
            };
            callbacks.credentials(move |_url, _username_from_url, _allowed_types| {
                Cred::ssh_key(
                    "git",
                    None,
                    std::path::Path::new(&ssh_private_key_path),
                    None,
                )
            });
        }
        AuthType::Pat => {
            let token = match secret_manager::get_personal_access_token() {
                Ok(token) => token,
                Err(_) => {
                    println!("Failed to get the personal access token");
                    return Err(git2::Error::from_str(
                        "Failed to get the personal access token",
                    ));
                }
            };
            callbacks.credentials(move |_url, _, _allowed_types| {
                Cred::userpass_plaintext(&token, &token)
            });
        }
    };

    let mut push_options = PushOptions::new();
    push_options.remote_callbacks(callbacks);

    remote.push(
        &[&format!("refs/heads/{}", branch_name)],
        Some(&mut push_options),
    )?;

    Ok(())
}
