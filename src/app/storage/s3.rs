use crate::app::config::S3;
use tokio::runtime::Runtime;

pub fn upload_to_s3(s3_config: &S3, file_path: &str, file_name: &str) -> bool {
    println!("Uploading to S3 bucket: {}", s3_config.bucket);
    
    // Create a tokio runtime for async S3 operations
    let runtime = match Runtime::new() {
        Ok(rt) => rt,
        Err(e) => {
            println!("Failed to create Tokio runtime: {}", e);
            return false;
        }
    };
    
    runtime.block_on(async {
        // Load AWS credentials
        let aws_config = aws_config::from_env()
            .credentials_provider(
                aws_credential_types::Credentials::new(
                    &s3_config.access_key,
                    &s3_config.secret_key,
                    None, None, "backup-app"
                )
            )
            .load()
            .await;
        
        // Create S3 client
        let client = aws_sdk_s3::Client::new(&aws_config);
        
        // Read the file
        match std::fs::read(file_path) {
            Ok(data) => {
                // Construct the full path in S3
                let s3_path = if s3_config.path.ends_with('/') {
                    format!("{}{}", s3_config.path, file_name)
                } else {
                    format!("{}/{}", s3_config.path, file_name)
                };
                
                // Upload the file to S3
                match client.put_object()
                    .bucket(&s3_config.bucket)
                    .key(s3_path)
                    .body(data.into())
                    .send()
                    .await
                {
                    Ok(_) => {
                        println!("File uploaded to S3 successfully");
                        return true;
                    },
                    Err(e) => {
                        println!("Failed to upload file to S3: {}", e);
                        return false;
                    }
                }
            },
            Err(e) => {
                println!("Failed to read file for S3 upload: {}", e);
                false
            }
        }
    })
}
