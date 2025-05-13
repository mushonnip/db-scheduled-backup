use std::fs::File;
use std::io::Read;
use std::path::Path;
use crate::app::config::Ftp;
use suppaftp::FtpStream;

pub fn upload_to_ftp(ftp_config: &Ftp, file_path: &str, file_name: &str) -> bool {
    println!("Uploading to FTP: {}:{}", ftp_config.host, ftp_config.port);
    
    let mut ftp_stream = match FtpStream::connect(format!("{}:{}", ftp_config.host, ftp_config.port)) {
        Ok(stream) => stream,
        Err(e) => {
            println!("Failed to connect to FTP server: {}", e);
            return false;
        }
    };
    
    match ftp_stream.login(&ftp_config.username, &ftp_config.password) {
        Ok(_) => println!("FTP login successful"),
        Err(e) => {
            println!("Failed to login to FTP server: {}", e);
            return false;
        }
    }
    
    // Navigate to the configured path
    let remote_path = Path::new(&ftp_config.path);
    if !remote_path.as_os_str().is_empty() {
        // Create directory structure if it doesn't exist
        for component in remote_path.components() {
            let dir_name = component.as_os_str().to_string_lossy();
            if !dir_name.is_empty() {
                let _ = ftp_stream.mkdir(&dir_name.to_string());
                if let Err(e) = ftp_stream.cwd(&dir_name.to_string()) {
                    println!("Failed to change directory: {}", e);
                    return false;
                }
            }
        }
    }
    
    // Open the file to upload
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            println!("Failed to open file for upload: {}", e);
            return false;
        }
    };
    
    // Read file contents
    let mut contents = Vec::new();
    if let Err(e) = file.read_to_end(&mut contents) {
        println!("Failed to read file contents: {}", e);
        return false;
    }
    
    // Upload the file
    match ftp_stream.put(file_name, &contents) {
        Ok(_) => {
            println!("File uploaded to FTP successfully");
            true
        },
        Err(e) => {
            println!("Failed to upload file to FTP: {}", e);
            false
        }
    }
}
