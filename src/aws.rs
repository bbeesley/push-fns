use crate::{
  args::AWSArgs,
  search::search,
  upload::aws_s3::s3_upload,
  zip::{create_zip, SymLink},
};

/// Zips up function assets and uploads them to AWS S3 for use in lambda functions.
/// Optionally creates a file for a layer as well as a file for the function itself.
///
/// Example
/// ```rust
/// use push_fn_lib::aws::{push_aws, AWSArgs};
///
/// async fn do_upload() {
///   let version = "1.0.0".to_string();
///
///   let args = AWSArgs {
///     regions: vec!["eu-west-2".to_string()],
///     buckets: vec!["fn-push-testing".to_string()],
///     function_key: "aws-test".to_string(),
///     include: vec!["src/aws.rs".to_string()],
///     exclude: vec![],
///     input_path: ".".to_string(),
///     layer_key: None,
///     layer_globs: vec![],
///     version_suffix: Some(version),
///     root_dir: None,
///     symlink_node_modules: false,
///   };
///   push_aws(args).await;
/// }
/// ```
pub async fn push_aws(args: AWSArgs) {
  let fn_object_key = match args.version_suffix.clone() {
    Some(version) => format!("{}-{}.zip", args.function_key, version),
    None => format!("{}.zip", args.function_key),
  };
  let mut exclude = args.exclude.clone();
  if args.layer_key.is_some() {
    exclude.append(args.layer_globs.clone().as_mut());
    let layer_object_key = match args.version_suffix {
      Some(version) => format!("{}-{}.zip", args.layer_key.unwrap(), version),
      None => format!("{}.zip", args.layer_key.unwrap()),
    };
    for (ix, bucket) in args.buckets.iter().enumerate() {
      let file_list = search(&args.input_path, &args.layer_globs, &[]);
      let buffer = create_zip(&args.input_path, file_list, None);
      s3_upload(&args.regions[ix], bucket, &layer_object_key, buffer).await;
    }
  }
  for (ix, bucket) in args.buckets.iter().enumerate() {
    let file_list = search(&args.input_path, &args.include, &exclude);
    let buffer = create_zip(
      &args.input_path,
      file_list,
      match args.symlink_node_modules {
        true => Some(SymLink {
          target: "/opt/nodejs/node_modules".to_string(),
          path: "node_modules".to_string(),
        }),
        false => None,
      },
    );
    s3_upload(&args.regions[ix], bucket, &fn_object_key, buffer).await;
  }
}

#[cfg(test)]
mod tests {
  use aws_sdk_s3::{config::Region, Client};
  use rand::Rng;
  use zip::ZipArchive;

  use super::*;
  use std::{
    fs::File,
    io::{self, Cursor, Read},
  };

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
  async fn aws_uploader_works_with_version() {
    let random_string = generate_random_string(10);

    let args = AWSArgs {
      regions: vec!["eu-west-2".to_string()],
      buckets: vec!["fn-push-testing".to_string()],
      function_key: "aws-test".to_string(),
      include: vec!["src/aws.rs".to_string()],
      exclude: vec![],
      input_path: ".".to_string(),
      layer_key: None,
      layer_globs: vec![],
      version_suffix: Some(random_string.clone()),
      root_dir: None,
      symlink_node_modules: false,
    };
    push_aws(args).await;
    let file_name = "src/aws.rs";
    let object_name = format!("aws-test-{}.zip", random_string);
    println!("object_name: {}", object_name);
    let region = "eu-west-2".to_string();
    let bucket = "fn-push-testing".to_string();
    let region_provider = Region::new(region.to_owned());
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    let zip_object = client
      .get_object()
      .bucket(bucket)
      .key(object_name)
      .send()
      .await
      .unwrap()
      .body
      .collect()
      .await
      .unwrap()
      .into_bytes();

    // Create a buffer to hold the file from the zip
    let mut content_buf = Cursor::new(Vec::new());

    // Open the zip archive for reading
    let mut zip = ZipArchive::new(Cursor::new(zip_object)).unwrap();

    // Extract the file from the archive
    let mut file_entry = zip.by_name(file_name).unwrap();

    // Copy it to the buffer
    io::copy(&mut file_entry, &mut content_buf).unwrap();

    // Read the same file in directly for an expected value
    let mut file = File::open(file_name).unwrap();

    // We'll store it in this buffer
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();

    // Compare the content we got from the archive to the content we got by reading from disk
    assert_eq!(file_buf, content_buf.into_inner());
  }

  #[tokio::test]
  async fn aws_uploader_works_without_version() {
    let function_key = generate_random_string(10);

    let args = AWSArgs {
      regions: vec!["eu-west-2".to_string()],
      buckets: vec!["fn-push-testing".to_string()],
      function_key: function_key.clone(),
      include: vec!["src/aws.rs".to_string()],
      exclude: vec![],
      input_path: ".".to_string(),
      layer_key: None,
      layer_globs: vec![],
      version_suffix: None,
      root_dir: None,
      symlink_node_modules: false,
    };
    push_aws(args).await;
    let file_name = "src/aws.rs";
    let object_name = format!("{}.zip", function_key);
    println!("object_name: {}", object_name);
    let region = "eu-west-2".to_string();
    let bucket = "fn-push-testing".to_string();
    let region_provider = Region::new(region.to_owned());
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    let zip_object = client
      .get_object()
      .bucket(bucket)
      .key(object_name)
      .send()
      .await
      .unwrap()
      .body
      .collect()
      .await
      .unwrap()
      .into_bytes();

    // Create a buffer to hold the file from the zip
    let mut content_buf = Cursor::new(Vec::new());

    // Open the zip archive for reading
    let mut zip = ZipArchive::new(Cursor::new(zip_object)).unwrap();

    // Extract the file from the archive
    let mut file_entry = zip.by_name(file_name).unwrap();

    // Copy it to the buffer
    io::copy(&mut file_entry, &mut content_buf).unwrap();

    // Read the same file in directly for an expected value
    let mut file = File::open(file_name).unwrap();

    // We'll store it in this buffer
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();

    // Compare the content we got from the archive to the content we got by reading from disk
    assert_eq!(file_buf, content_buf.into_inner());
  }

  #[tokio::test]
  async fn aws_uploader_works_with_layers() {
    let random_string = generate_random_string(10);

    let args = AWSArgs {
      regions: vec!["eu-west-2".to_string()],
      buckets: vec!["fn-push-testing".to_string()],
      function_key: "aws-test".to_string(),
      include: vec!["src/*.rs".to_string()],
      exclude: vec![],
      input_path: ".".to_string(),
      layer_key: Some("aws-layer".to_string()),
      layer_globs: vec!["src/aws.rs".to_string()],
      version_suffix: Some(random_string.clone()),
      root_dir: None,
      symlink_node_modules: false,
    };
    push_aws(args).await;
    let file_name = "src/aws.rs";
    let fn_object_name = format!("aws-test-{}.zip", random_string);
    let layer_object_name = format!("aws-layer-{}.zip", random_string);
    println!("object_name: {}", fn_object_name);
    let region = "eu-west-2".to_string();
    let bucket = "fn-push-testing".to_string();
    let region_provider = Region::new(region.to_owned());
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    let layer_zip_object = client
      .get_object()
      .bucket(bucket.clone())
      .key(layer_object_name)
      .send()
      .await
      .unwrap()
      .body
      .collect()
      .await
      .unwrap()
      .into_bytes();

    let fn_zip_object = client
      .get_object()
      .bucket(bucket.clone())
      .key(fn_object_name)
      .send()
      .await
      .unwrap()
      .body
      .collect()
      .await
      .unwrap()
      .into_bytes();

    // Create a buffer to hold the file from the layer zip
    let mut layer_content_buf: Cursor<Vec<u8>> = Cursor::new(Vec::new());

    // Open the zip archive for reading
    let mut layer_zip = ZipArchive::new(Cursor::new(layer_zip_object)).unwrap();

    // Extract the file from the archive
    let mut file_entry = layer_zip.by_name(file_name).unwrap();

    // Copy it to the buffer
    io::copy(&mut file_entry, &mut layer_content_buf).unwrap();

    // Read the same file in directly for an expected value
    let mut file = File::open(file_name).unwrap();

    // We'll store it in this buffer
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();

    // Compare the content we got from the archive to the content we got by reading from disk
    assert_eq!(file_buf, layer_content_buf.into_inner());

    // Open the zip archive for reading
    let mut fn_zip = ZipArchive::new(Cursor::new(fn_zip_object)).unwrap();

    // Extract the file from the archive
    let fn_file_entry = fn_zip.by_name(file_name);

    assert!(fn_file_entry.is_err());
  }
}
