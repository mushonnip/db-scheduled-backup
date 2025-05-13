use crate::app::config::S3;
use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;
use std::path::Path;
use tokio::runtime::Runtime;

pub fn upload_to_s3(s3_config: &S3, file_path: &str, file_name: &str) -> bool {
    println!("Uploading to MinIO/S3 bucket: {}", s3_config.bucket);

    // Create a tokio runtime for async S3 operations
    let runtime = match Runtime::new() {
        Ok(rt) => rt,
        Err(e) => {
            println!("Failed to create Tokio runtime: {}", e);
            return false;
        }
    };

    runtime.block_on(async {
        // Get region using the endpoint from config
        let region = Region::Custom {
            region: "us-east-1".to_owned(), // Default region for MinIO, can be anything
            endpoint: s3_config.endpoint.clone(),
        };

        // Create credentials
        let credentials = Credentials::new(
            Some(&s3_config.access_key),
            Some(&s3_config.secret_key),
            None,
            None,
            None,
        )
        .unwrap();

        // Create bucket handle
        let bucket = Bucket::new(&s3_config.bucket, region, credentials)
            .expect("Failed to create bucket handle")
            .with_path_style();

        // Read the file
        match std::fs::read(file_path) {
            Ok(data) => {
                // Construct the full path in S3/MinIO
                let s3_path = if s3_config.path.ends_with('/') {
                    format!("{}{}", s3_config.path, file_name)
                } else {
                    format!("{}/{}", s3_config.path, file_name)
                };

                // Get content type based on file extension
                let content_type = match Path::new(file_name).extension() {
                    Some(ext) => match ext.to_str().unwrap_or("") {
                        "gz" => "application/gzip",
                        "zip" => "application/zip",
                        "tar" => "application/x-tar",
                        "sql" => "application/sql",
                        _ => "application/octet-stream",
                    },
                    None => "application/octet-stream",
                };

                // Upload the file to S3/MinIO
                match bucket
                    .put_object_with_content_type(&s3_path, &data, content_type)
                    .await
                {
                    Ok(_) => {
                        println!("File uploaded to MinIO/S3 successfully");
                        true
                    }
                    Err(e) => {
                        println!("Failed to upload file to MinIO/S3: {}", e);
                        false
                    }
                }
            }
            Err(e) => {
                println!("Failed to read file for MinIO/S3 upload: {}", e);
                false
            }
        }
    })
}
