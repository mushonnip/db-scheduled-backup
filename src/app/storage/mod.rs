pub mod ftp;
pub mod s3;

use crate::app::config::{Ftp, S3, Storage};

pub fn upload_backup(storage: &Storage, file_path: &str, file_name: &str) -> bool {
    match storage.media.as_str() {
        "ftp" => ftp::upload_to_ftp(storage.ftp.as_ref().unwrap(), file_path, file_name),
        "s3" => s3::upload_to_s3(storage.s3.as_ref().unwrap(), file_path, file_name),
        _ => {
            println!("Unsupported storage media: {}", storage.media);
            false
        }
    }
}
