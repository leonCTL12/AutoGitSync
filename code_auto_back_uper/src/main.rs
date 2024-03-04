//This is to test if the ignore crate work properly

use std::path::Path;

use ignore::gitignore::Gitignore;

fn main() {
    let git_ignore = create_gitignore("/Users/leonchan/Desktop/test_git_ignore");
    let git_ignore = match git_ignore {
        Ok(git_ignore) => git_ignore,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    test_git_ignore(
        &git_ignore,
        &String::from("../test_git_ignore"),
        &String::from("../test_git_ignore/a.txt"),
    );

    test_git_ignore(
        &git_ignore,
        &String::from("../test_git_ignore"),
        &String::from("../test_git_ignore/log/b.mp4"),
    );

    test_git_ignore(
        &git_ignore,
        &String::from("../test_git_ignore"),
        &String::from("../test_git_ignore/c.png"),
    );
}

fn create_gitignore(path: &str) -> Result<Gitignore, String> {
    let gitignore_path = Path::new(path).join(".gitignore");
    let var_name = Gitignore::new(gitignore_path);
    let git_ignore = var_name;

    if git_ignore.1.is_some() {
        return Err(format!("Failed to create gitignore for {}", path));
    }

    let git_ignore = git_ignore.0;
    Ok(git_ignore)
}

pub fn test_git_ignore(git_ignore: &Gitignore, repo_path: &String, file_path: &String) {
    let absolute_path = Path::new(file_path);
    let relative_path = absolute_path.strip_prefix(repo_path).unwrap();

    let is_ignored = git_ignore.matched(relative_path, false).is_ignore();

    if is_ignored {
        println!("{} is ignored", file_path);
    } else {
        println!("{} is not ignored", file_path);
    }
}
