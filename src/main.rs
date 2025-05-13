mod app;

use chrono::Local;
use cron::Schedule;
use std::str::FromStr;
use std::thread;

fn main() {
    let config = app::config::get_config();

    // let expression = String::from("0/5 * * * * *"); // every 5 seconds
    let expression = config
        .cron
        .map(|cron| cron.expression)
        .unwrap_or_else(|| String::from("0 30 0 * * *")); // dafault is daily at 00:30

    println!("Using cron expression: {}", expression);
    let schedule =
        Schedule::from_str(expression.as_str()).expect("Failed to parse CRON expression");

    println!("Upcoming fire times:");
    for datetime in schedule.upcoming(Local).take(5) {
        println!("-> {}", datetime);
    }

    loop {
        let now = Local::now();
        if let Some(next) = schedule.upcoming(Local).take(1).next() {
            let until_next = next - now;
            thread::sleep(until_next.to_std().unwrap());

            app::backup::backup();
        }
    }
}
