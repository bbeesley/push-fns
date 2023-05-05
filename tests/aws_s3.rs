
use std::{fs::File, io::Read};

use aws_sdk_s3::{config::Region, Client};
use push_fns::upload::aws_s3::s3_upload;

#[tokio::test]
async fn upload_works_properly() {
    let file_name = "src/zip.rs";
    let region = "eu-west-2".to_string();
    let bucket = "fn-push-testing".to_string();
    let region_provider = Region::new(region.to_owned());
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    // Read the same file in directly for an expected value
    let mut file = File::open(file_name).unwrap();

    // We'll store it in this buffer
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();

    s3_upload(&region, &bucket, &file_name.to_string(), file_buf.clone()).await;

    let object = client
        .get_object()
        .bucket(bucket)
        .key(file_name)
        .send()
        .await
        .unwrap();
    assert_eq!(file_buf, object.body.collect().await.unwrap().into_bytes())
}
