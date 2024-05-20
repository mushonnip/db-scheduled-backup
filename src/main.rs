mod app;

use chrono::Local;
use cron::Schedule;
use std::env;
use std::process::Command;
use std::str::FromStr;
use std::thread;

fn backup() {
    let config = app::config::get_config();
    let current_dir = env::current_dir().expect("Failed to get current directory");

    let file_name = Local::now().format("backup-%d-%m-%Y.sql.gz").to_string();
    let db_container_name = config.database.db_container_name;
    let db_username = config.database.db_username;
    let db_name = config.database.db_name;
    let output_file = format!("{}/file/{}", current_dir.to_str().unwrap(), file_name);

    println!("Backing up now: {}", file_name);
    let command = format!(
        r#"sudo docker exec -i "{}" /usr/bin/pg_dump -U "{}" "{}" | gzip > "{}""#,
        db_container_name, db_username, db_name, output_file
    );

    let output = Command::new("sh")
        .arg("-c")
        .arg(&command)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!("Command executed successfully");
        remove_previous_backup(&file_name);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Command failed with error: {}", stderr);
    }
}

fn remove_previous_backup(file_name_now: &str) {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let path = format!("{}/file", current_dir.to_str().unwrap());

    // delete files except a file named file_name_now when prefixed with "backup-" and suffixed with ".sql.gz"
    for entry in std::fs::read_dir(path).expect("Failed to read directory") {
        let entry = entry.expect("Failed to get entry");
        let file_name = entry.file_name().into_string().unwrap();
        if file_name != file_name_now
            && file_name.starts_with("backup-")
            && file_name.ends_with(".sql.gz")
        {
            std::fs::remove_file(entry.path()).expect("Failed to remove file");
        }
    }
}
fn main() {
    // let expression = "0/5 * * * * *"; // every 5 seconds

    let expression = "0 30 0 * * *"; // daily at 00:30
    let schedule = Schedule::from_str(expression).expect("Failed to parse CRON expression");

    // println!("Upcoming fire times:");
    // for datetime in schedule.upcoming(Local).take(10) {
    //     println!("-> {}", datetime);
    // }

    loop {
        let now = Local::now();
        if let Some(next) = schedule.upcoming(Local).take(1).next() {
            let until_next = next - now;
            thread::sleep(until_next.to_std().unwrap());

            backup();
        }
    }
}
