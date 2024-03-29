use crate::config_manager;
use chrono::{DateTime, Utc};
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashSet;
use std::path::Path;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

//This whole file watcher script will be run in a separate thread

pub struct FileChangeSignal {
    pub paths: Vec<String>,
    pub timestamp: DateTime<Utc>,
}

impl FileChangeSignal {
    pub fn new(path: Vec<String>, timestamp: DateTime<Utc>) -> FileChangeSignal {
        FileChangeSignal {
            paths: path,
            timestamp,
        }
    }
}

fn create_file_watcher(
    tx: Sender<FileChangeSignal>,
) -> Result<notify::RecommendedWatcher, notify::Error> {
    let tx = tx.clone();
    notify::recommended_watcher(move |res| match res {
        Ok(event) => on_file_change_event(event, tx.clone()),
        Err(e) => println!("watch error: {:?}", e),
    })
}

fn update_watching_repo(
    watcher: &mut RecommendedWatcher,
    previous_watching_folder: &mut HashSet<String>,
) -> notify::Result<()> {
    let config = config_manager::read_config();
    let current_watching_folder = config.watching_folders.iter().cloned().collect();

    for folder in previous_watching_folder.difference(&current_watching_folder) {
        watcher.unwatch(Path::new(&folder))?;
    }

    for folder in current_watching_folder.difference(previous_watching_folder) {
        watcher.watch(Path::new(&folder), RecursiveMode::Recursive)?;
    }

    *previous_watching_folder = current_watching_folder;
    Ok(())
}

fn on_file_change_event(event: Event, tx: Sender<FileChangeSignal>) {
    let mut paths: Vec<String> = Vec::new();
    for path in &event.paths {
        if let Some(path) = path.to_str() {
            if path.contains(".git") {
                return;
            }
            paths.push(path.to_string());
        }
    }

    let signal = FileChangeSignal::new(paths, Utc::now());

    tx.send(signal).unwrap();
}

pub fn start(tx: Sender<FileChangeSignal>) {
    thread::spawn(move || {
        let mut watcher = match create_file_watcher(tx) {
            Ok(watcher) => watcher,
            Err(e) => {
                println!("start watcher failed");
                println!("Error: {:?}", e);
                return;
            }
        };
        let mut watched_folder: HashSet<String> = HashSet::new();
        loop {
            if let Err(e) = update_watching_repo(&mut watcher, &mut watched_folder) {
                println!("watch repository failed");
                println!("Error: {:?}", e);
                return;
            }
            std::thread::sleep(Duration::from_millis(5000));
        }
    });
}
