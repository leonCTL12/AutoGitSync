use crate::config_manager;
use chrono::{DateTime, Utc};
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
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

fn watch_directories(watcher: &mut RecommendedWatcher) -> notify::Result<()> {
    let config = config_manager::read_config();
    for folder in config.watching_folders {
        watcher.watch(Path::new(&folder), RecursiveMode::Recursive)?;
    }
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

    //TODO: create a new file change signal!
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

        if let Err(e) = watch_directories(&mut watcher) {
            println!("start watcher failed");
            println!("Error: {:?}", e);
            return;
        }

        loop {
            std::thread::sleep(Duration::from_millis(5000));
        }
    });
}
