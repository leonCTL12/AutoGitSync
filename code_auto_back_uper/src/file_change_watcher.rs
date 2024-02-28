use notify::{RecursiveMode, Watcher};
use std::path::Path;
use std::time::Duration;

pub fn start() {
    std::thread::spawn(move || {
        let mut watcher = match notify::recommended_watcher(|res| match res {
            Ok(event) => println!("event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }) {
            Ok(watcher) => watcher,
            Err(e) => {
                println!("start watcher failed");
                println!("Error: {:?}", e);
                return;
            }
        };

        if let Err(e) = watcher.watch(
            Path::new("/Users/leonchan/WorkSpace/git_auto_sync_test_repo"),
            RecursiveMode::Recursive,
        ) {
            println!("start watcher failed");
            println!("Error: {:?}", e);
            return;
        }

        loop {
            std::thread::sleep(Duration::from_millis(5000));
        }
    });
}
