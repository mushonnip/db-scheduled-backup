use std::env;
use std::process::Command;
use chrono::Local;
use crate::app::config;
use crate::app::storage;

pub fn backup() {
    let config = config::get_config();
    let current_dir = env::current_dir().expect("Failed to get current directory");

    let file_name = Local::now().format("backup-%d-%m-%Y-%H-%M-%S.sql.gz").to_string();
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
        let upload_success = storage::upload_backup(&config.storage, &output_file, &file_name);
        if upload_success {
            println!("Backup file uploaded successfully");
        }
        remove_previous_backup(&file_name);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Command failed with error: {}", stderr);
    }
}

pub fn remove_previous_backup(file_name_now: &str) {
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
