mod app;

use chrono::Local;
use cron::Schedule;
use std::str::FromStr;
use std::thread;

fn main() {
    // let expression = "0/5 * * * * *"; // every 5 seconds

    let expression = "0 30 0 * * *"; // daily at 00:30
    let schedule = Schedule::from_str(expression).expect("Failed to parse CRON expression");

    // println!("Upcoming fire times:");
    // for datetime in schedule.upcoming(Local).take(10) {
    //     println!("-> {}", datetime);
    // }

    let config = app::config::get_config();

    loop {
        let now = Local::now();
        if let Some(next) = schedule.upcoming(Local).take(1).next() {
            let until_next = next - now;
            thread::sleep(until_next.to_std().unwrap());
            println!(
                "Backing up now: {}",
                Local::now().format("%Y-%m-%d %H:%M:%S")
            );
        }
    }
}
