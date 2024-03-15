use notify_rust::Notification;

fn main() {
    let _ = Notification::new()
        .summary("Your New Title") // Change this line
        .body("This is a notification from Rust!")
        .show();
}
