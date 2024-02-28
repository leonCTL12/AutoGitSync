use notify::{Event, RecommendedWatcher, RecursiveMode, Result, Watcher};
use std::time::Duration;

fn main() -> Result<()> {
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
        // Sleep for a short duration to prevent the loop from running at full speed.
        std::thread::sleep(Duration::from_millis(100));
    }
}
