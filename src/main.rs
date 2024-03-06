use gitignored::Gitignore;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::{io, path::PathBuf};
fn main() {
    println!("{}", env::current_dir().unwrap().display());
    let mut ig = Gitignore::new(
        Path::new("/Users/leonchan/Workspace/AutoGitSync"),
        true,
        true,
    );

    let ignored_pattern =
        read_git_ignore_file("/Users/leonchan/Workspace/AutoGitSync/.gitignore").unwrap();
    let slice_of_str: Vec<&str> = ignored_pattern.iter().map(|s| s.as_str()).collect();
    let slice_of_str: &[&str] = &slice_of_str;

    // Define the paths you want to check
    let paths = vec![
        "src/main.rs",
       "target/debug/.fingerprint/telepaste_be-edd20ff822cb5555/bin-telepaste_be",
       "target/debug/incremental/telepaste_be-2u8j4f837om8j/s-gu0355ge82-1qhet2m-95p2fc2jv0gikk6zg2eqyx22r",
       "target/test/a.png",
    ];

    // Check each path
    for path in paths {
        if ig.ignores(&slice_of_str, ig.root.join(&path)) {
            println!("{} is ignored", ig.root.join(&path).display());
        } else {
            println!("{} is not ignored", ig.root.join(&path).display());
        }
    }
}
fn read_git_ignore_file(file_path: &str) -> Result<Vec<String>, String> {
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => return Err(format!("Failed to open {}", file_path)),
    };
    let reader = io::BufReader::new(file);

    let lines: Vec<String> = reader
        .lines()
        .filter_map(|line| {
            let line = line.ok()?;
            if line.starts_with('#') || line.is_empty() {
                None
            } else {
                Some(line)
            }
        })
        .collect();

    Ok(lines)
}
