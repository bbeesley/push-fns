use crate::{
  args::GCPArgs, search::search, upload::google_cloud_storage::cs_upload, zip::create_zip,
};

/// Zips up function assets and uploads them to Google Cloud Storage for use in Cloud Functions.
///
/// Example
/// ```rust
/// use google_cloud_default::WithAuthExt;
/// use google_cloud_storage::{client::{ClientConfig, Client}, http::objects::{get::GetObjectRequest, download::Range}};
/// use push_fn_lib::gcp::{GCPArgs, push_gcs};
///
/// async fn do_upload() {
///   let version = "1.0.0".to_string();
///   let args = GCPArgs {
///     buckets: vec!["fn-push-testing".to_string()],
///     function_key: "gcp-test".to_string(),
///     include: vec!["src/gcp.rs".to_string()],
///     exclude: vec![],
///     input_path: ".".to_string(),
///     version_suffix: Some(version.clone()),
///     root_dir: None,
///   };
///   push_gcs(args).await;
/// }
/// ```
pub async fn push_gcs(args: GCPArgs) {
  let fn_object_key = match args.version_suffix.clone() {
    Some(version) => format!("{}-{}.zip", args.function_key, version),
    None => format!("{}.zip", args.function_key),
  };
  for bucket in args.buckets.iter() {
    let file_list = search(&args.input_path, &args.include, &args.exclude);
    let buffer = create_zip(&args.input_path, file_list, None);
    cs_upload(bucket, &fn_object_key, buffer).await;
  }
}

#[cfg(test)]
mod tests {
  use google_cloud_default::WithAuthExt;
  use google_cloud_storage::{
    client::{Client, ClientConfig},
    http::objects::{download::Range, get::GetObjectRequest},
  };
  use rand::Rng;
  use zip::ZipArchive;

  use super::*;
  use std::{
    env,
    fs::File,
    io::{self, Cursor, Read},
  };

  fn get_file_path() -> String {
    let path = match env::consts::OS {
      "windows" => "src\\gcp.rs",
      _ => "src/gcp.rs",
    };
    path.to_string()
  }

  fn generate_random_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let random_string: String = (0..length)
      .map(|_| {
        let idx = rng.gen_range(0..charset.len());
        charset[idx] as char
      })
      .collect();
    random_string
  }
  #[tokio::test]
  async fn gcs_uploader_works_with_version() {
    let random_string = generate_random_string(10);

    let args = GCPArgs {
      buckets: vec!["fn-push-testing".to_string()],
      function_key: "gcp-test".to_string(),
      include: vec![get_file_path()],
      exclude: vec![],
      input_path: ".".to_string(),
      version_suffix: Some(random_string.clone()),
      root_dir: None,
    };
    push_gcs(args).await;
    let object = format!("gcp-test-{}.zip", random_string);
    println!("object_name: {}", object);
    let bucket = "fn-push-testing".to_string();
    let config = ClientConfig::default().with_auth().await;
    let client: Client = match config {
      Ok(c) => Client::new(c),
      Err(e) => {
        println!("Error: {}", e);
        panic!();
      }
    };

    let zip_object = client
      .download_object(
        &GetObjectRequest {
          bucket,
          object,
          ..Default::default()
        },
        &Range::default(),
      )
      .await
      .unwrap();

    // Create a buffer to hold the file from the zip
    let mut content_buf = Cursor::new(Vec::new());

    // Open the zip archive for reading
    let mut zip = ZipArchive::new(Cursor::new(zip_object)).unwrap();

    // Extract the file from the archive
    let mut file_entry = zip.by_name(get_file_path().as_str()).unwrap();

    // Copy it to the buffer
    io::copy(&mut file_entry, &mut content_buf).unwrap();

    // Read the same file in directly for an expected value
    let mut file = File::open(get_file_path().as_str()).unwrap();

    // We'll store it in this buffer
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();

    // Compare the content we got from the archive to the content we got by reading from disk
    assert_eq!(file_buf, content_buf.into_inner());
  }

  #[tokio::test]
  async fn gcs_uploader_works_without_version() {
    let function_key = generate_random_string(10);

    let args = GCPArgs {
      buckets: vec!["fn-push-testing".to_string()],
      function_key: function_key.clone(),
      include: vec![get_file_path()],
      exclude: vec![],
      input_path: ".".to_string(),
      version_suffix: None,
      root_dir: None,
    };
    push_gcs(args).await;
    let object = format!("{}.zip", function_key);
    println!("object_name: {}", object);
    let bucket = "fn-push-testing".to_string();
    let config = ClientConfig::default().with_auth().await;
    let client: Client = match config {
      Ok(c) => Client::new(c),
      Err(e) => {
        println!("Error: {}", e);
        panic!();
      }
    };

    let zip_object = client
      .download_object(
        &GetObjectRequest {
          bucket,
          object,
          ..Default::default()
        },
        &Range::default(),
      )
      .await
      .unwrap();

    // Create a buffer to hold the file from the zip
    let mut content_buf = Cursor::new(Vec::new());

    // Open the zip archive for reading
    let mut zip = ZipArchive::new(Cursor::new(zip_object)).unwrap();

    // Extract the file from the archive
    let mut file_entry = zip.by_name(get_file_path().as_str()).unwrap();

    // Copy it to the buffer
    io::copy(&mut file_entry, &mut content_buf).unwrap();

    // Read the same file in directly for an expected value
    let mut file = File::open(get_file_path().as_str()).unwrap();

    // We'll store it in this buffer
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();

    // Compare the content we got from the archive to the content we got by reading from disk
    assert_eq!(file_buf, content_buf.into_inner());
  }
}
