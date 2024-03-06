use gitignored::Gitignore;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::{io, path::PathBuf};
mod gitignore_wrapper;
use gitignore_wrapper::GitIgnoreWrapper;
fn main() {
    let mut gitignore_wrapper =
        GitIgnoreWrapper::new(PathBuf::from("/Users/leonchan/Workspace/AutoGitSync"));

    // Define the paths you want to check
    let paths = vec![
        "/Users/leonchan/Workspace/AutoGitSync/src/main.rs",
       "/Users/leonchan/Workspace/AutoGitSync/target/debug/.fingerprint/telepaste_be-edd20ff822cb5555/bin-telepaste_be",
       "/Users/leonchan/Workspace/AutoGitSync/target/debug/incremental/telepaste_be-2u8j4f837om8j/s-gu0355ge82-1qhet2m-95p2fc2jv0gikk6zg2eqyx22r",
       "/Users/leonchan/Workspace/AutoGitSync/target/test/a.png",
    ];

    // Check each path
    for path in paths {
        gitignore_wrapper.query(Path::new(&path));
    }
}
