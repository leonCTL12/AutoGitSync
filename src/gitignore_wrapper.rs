use gitignored::Gitignore;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::{io, path::PathBuf};

pub struct GitIgnoreWrapper {
    gitignore: Gitignore<PathBuf>,
    rules: Vec<String>,
}

impl GitIgnoreWrapper {
    pub fn new(repo_path: PathBuf) -> GitIgnoreWrapper {
        println!("Repo path: {}", repo_path.display());
        let ignore: Gitignore<PathBuf> = Gitignore::new(repo_path.clone(), true, true);
        let ignored_pattern = match extract_rules_from_gitignore(&repo_path) {
            Ok(ignored_pattern) => ignored_pattern,
            Err(e) => {
                panic!("Failed to extract rules from .gitignore: {}", e);
            }
        };

        GitIgnoreWrapper {
            gitignore: ignore,
            rules: ignored_pattern,
        }
    }

    pub fn query(&mut self, path: &Path) -> bool {
        //Convert Vec<String> to &[&str]
        let rules: Vec<&str> = self.rules.iter().map(|s| s.as_str()).collect();
        let rules: &[&str] = &rules;
        self.gitignore.ignores(rules, path)
    }
}

fn extract_rules_from_gitignore(repo_path: &Path) -> Result<Vec<String>, String> {
    let file = match File::open(repo_path.join(".gitignore")) {
        Ok(file) => file,
        Err(_) => {
            println!("No .gitignore found in {}", repo_path.display());
            return Ok(Vec::new());
        }
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
