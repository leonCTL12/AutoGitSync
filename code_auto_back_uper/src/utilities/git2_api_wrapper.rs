use git2::{Cred, PushOptions, RemoteCallbacks, Repository, Signature};

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
    let branch = repo.find_branch(branch_name, git2::BranchType::Local)?;
    let obj = branch.get().peel(git2::ObjectType::Commit)?;
    repo.checkout_tree(&obj, None)?;
    repo.set_head(&format!("refs/heads/{}", branch_name))?;
    Ok(())
}

pub fn apply_stash(repo: &mut Repository, delete_after_apply: bool) -> Result<(), git2::Error> {
    let stash_index = 0; //The latest stash
    let mut options = git2::StashApplyOptions::default();
    repo.stash_apply(stash_index, Some(&mut options))?;
    if delete_after_apply {
        repo.stash_drop(stash_index)?;
    }

    Ok(())
}

pub fn has_local_change(repo: &Repository) -> Result<bool, git2::Error> {
    //Check for changes
    let mut opts = git2::DiffOptions::new();
    let diff = repo.diff_index_to_workdir(None, Some(&mut opts))?;
    Ok(diff.deltas().count() > 0)
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

pub fn push_to_remote(repo: &Repository, branch_name: &str) -> Result<(), git2::Error> {
    let mut remote = repo.find_remote("origin")?;

    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::ssh_key(
            username_from_url.unwrap(),
            None,
            std::path::Path::new(&format!("{}/.ssh/id_rsa", std::env::var("HOME").unwrap())),
            None,
        )
    });

    let mut push_options = PushOptions::new();
    push_options.remote_callbacks(callbacks);

    remote.push(
        &[&format!("refs/heads/{}", branch_name)],
        Some(&mut push_options),
    )?;

    Ok(())
}