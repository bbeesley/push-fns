use std::{fs::File, io::Read};

use google_cloud_default::WithAuthExt;
use google_cloud_storage::client::{Client, ClientConfig};
use google_cloud_storage::http::objects::download::Range;
use google_cloud_storage::http::objects::get::GetObjectRequest;

use push_fns::upload::google_cloud_storage::cs_upload;

#[tokio::test]
async fn upload_works_properly() {
    let file_name = "src/upload/google_cloud_storage.rs";
    let bucket = "fn-push-testing".to_string();
    let config = ClientConfig::default().with_auth().await;
    let client: Client = match config {
        Ok(c) => Client::new(c),
        Err(e) => {
            println!("Error: {}", e);
            panic!();
        }
    };

    // Read the same file in directly for an expected value
    let mut file = File::open(file_name).unwrap();

    // We'll store it in this buffer
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();

    cs_upload(&bucket, &file_name.to_string(), file_buf.clone()).await;

    let data = client
        .download_object(
            &GetObjectRequest {
                bucket,
                object: file_name.to_string(),
                ..Default::default()
            },
            &Range::default(),
        )
        .await;
    assert_eq!(file_buf, data.unwrap())
}
