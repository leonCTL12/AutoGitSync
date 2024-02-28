use crate::config_manager;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::thread;
use std::time::Duration;

//This whole file watcher script will be run in a separate thread

fn create_file_watcher() -> Result<notify::RecommendedWatcher, notify::Error> {
    notify::recommended_watcher(|res| match res {
        Ok(event) => println!("event: {:?}", event),
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

pub fn start() {
    thread::spawn(move || {
        let mut watcher = match create_file_watcher() {
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
