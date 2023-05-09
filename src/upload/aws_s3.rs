use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::{config::Region, Client};

/// Uploads a buffer to AWS S3
///
/// # Examples
///
/// ```
/// use push_fns::upload::aws_s3::s3_upload;
/// use std::{fs::File, io::Read};
///
/// async fn do_something() {
///     let file_name = "src/zip.rs";
///     let region = "eu-west-2".to_string();
///     let bucket = "fn-push-testing".to_string();
///     let mut file = File::open(file_name).unwrap();
///     let mut file_buf = Vec::new();
///     file.read_to_end(&mut file_buf).unwrap();
///     s3_upload(&region, &bucket, &file_name.to_string(), file_buf.clone()).await;
/// }
/// ```
pub async fn s3_upload(region: &String, bucket: &String, key: &String, data: Vec<u8>) {
  let region_provider = Region::new(region.to_owned());
  let shared_config = aws_config::from_env().region(region_provider).load().await;
  let client = Client::new(&shared_config);
  let body = ByteStream::from(data);
  client
    .put_object()
    .bucket(bucket)
    .key(key)
    .body(body)
    .send()
    .await
    .unwrap();
}
