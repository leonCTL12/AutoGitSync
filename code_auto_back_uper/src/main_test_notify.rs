use notify::{Event, RecommendedWatcher, RecursiveMode, Result, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

fn main() -> Result<()> {
    // Create a channel to receive the events.
    let (tx, rx) = channel::<Result<Event>>();

    // Automatically select the best implementation for your platform.
    let mut watcher = notify::recommended_watcher(move |res| match res {
        Ok(event) => println!("event: {:?}", event),
        Err(e) => println!("watch error: {:?}", e),
    })?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(
        std::path::Path::new("/Users/leonchan/WorkSpace/git_auto_sync_test_repo"),
        RecursiveMode::Recursive,
    )?;

    loop {
        match rx.recv() {
            Ok(event) => println!("{:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
        // Sleep for a short duration to prevent the loop from running at full speed.
        std::thread::sleep(Duration::from_millis(100));
    }
}
