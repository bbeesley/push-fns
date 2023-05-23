use google_cloud_default::WithAuthExt;
use google_cloud_storage::client::{Client, ClientConfig};
use google_cloud_storage::http::objects::upload::{Media, UploadObjectRequest, UploadType};

/// Uploads a buffer to Google Cloud Storage.
///
/// # Examples
///
/// ```
/// use push_fn_lib::upload::google_cloud_storage::cs_upload;
/// use std::{fs::File, io::Read};
///
/// async fn do_something() {
///     let file_name = "src/upload/google_cloud_storage.rs";
///     let bucket = "fn-push-testing".to_string();
///     let mut file = File::open(file_name).unwrap();
///     let mut file_buf = Vec::new();
///     file.read_to_end(&mut file_buf).unwrap();
///     cs_upload(&bucket, &file_name.to_string(), file_buf.clone()).await;
/// }
/// ```
pub async fn cs_upload(bucket: &String, key: &String, data: Vec<u8>) {
  let config = ClientConfig::default().with_auth().await.unwrap();
  let client = Client::new(config);

  // Upload the file
  let upload_type = UploadType::Simple(Media::new(key.to_string()));
  client
    .upload_object(
      &UploadObjectRequest {
        bucket: bucket.to_string(),
        ..Default::default()
      },
      data,
      &upload_type,
    )
    .await
    .unwrap();
}

#[cfg(test)]
mod tests {
  use google_cloud_storage::http::objects::{download::Range, get::GetObjectRequest};

  use super::*;
  use std::{fs::File, io::Read};

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
}
