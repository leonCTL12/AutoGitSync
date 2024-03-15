use notify_rust::Notification;

pub fn show_notification(title: String, body: String) {
    match Notification::new().summary(&title).body(&body).show() {
        Ok(_) => {}
        Err(e) => {
            println!("Failed to show notification: {}", e);
        }
    }
}
