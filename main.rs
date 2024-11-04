use std::{env, thread, time};
use notify_rust::Notification;


fn format_duration(total_seconds: u64) -> String {
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    format!("{}:{}:{} H:M:S", hours, minutes, seconds)
}

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if both duration and interval are provided
    if args.len() != 3 {
        eprintln!("Usage: {} <duration_in_seconds> <notify_interval_in_seconds>", args[0]);
        return;
    }

    // Parse the duration
    let duration: u64 = match args[1].parse() {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Invalid duration. Please provide a number.");
            return;
        }
    };

    // Parse the notification interval
    let notify_interval: u64 = match args[2].parse() {
        Ok(i) => i,
        Err(_) => {
            eprintln!("Invalid notification interval. Please provide a number.");
            return;
        }
    };

    // Initial notification
    Notification::new()
        .summary("Start Timer!")
        .body(format!("{} notifications every {} seconds!", format_duration(duration),notify_interval).as_str())
        .icon("thunderbird")
        .timeout(5) // Notification will close after 5 seconds
        .show()
        .expect("Failed to send notification!");

    // Wait for the initial notification to finish
    thread::sleep(time::Duration::from_secs(5));

    let mut remaining_time = duration;
    

    while remaining_time > 0 {
        if remaining_time < 60 {
            Notification::new()
            .summary("IMPORTANT!")
            .body("Do terraform destroy!")
            .icon("thunderbird")
            .timeout(0)
            .show()
            .expect("Failed to send notification!");
        }
        // Calculate time to notify
        let notify_duration = remaining_time.min(notify_interval);
        
        // Wait for the notify interval or the remaining time
        thread::sleep(time::Duration::from_secs(notify_duration));
        remaining_time -= notify_duration;

        let minutes = remaining_time / 60;
        let seconds = remaining_time % 60;

        // Send notification with remaining time
        Notification::new()
            .summary("Count Down Timer")
            .body(format!("{} minutes and {} seconds remain.", minutes, seconds).as_str())
            .icon("thunderbird")
            .timeout(0) // Make the notification persistent
            .show()
            .expect("Failed to send notification!");
    }

    // Final notification when the countdown is over
    Notification::new()
        .summary("Time's Up!")
        .body("The countdown has finished!")
        .icon("thunderbird")
        .timeout(0)
        .show()
        .expect("Failed to send notification!");
}
