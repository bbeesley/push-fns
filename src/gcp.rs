use clap::Args;

use crate::{search::search, zip::create_zip, upload::google_cloud_storage::cs_upload};

#[derive(Args)]
pub struct GCPArgs {
    /// An array of globs defining what to bundle
    #[arg(short, long, default_values_t = [String::from("**")])]
    include: Vec<String>,
    
    /// An array of globs defining what not to bundle
    #[arg(short, long)]
    exclude: Vec<String>,
    
    /// A list of buckets to upload to (same order as the regions please)
    #[arg(short, long, required = true)]
    buckets: Vec<String>,

    /// The path/filename of the zip file in the bucket (you don't need to add the .zip extension)
    #[arg(short, long)]
    function_key: String,

    /// The path to the lambda code and node_modules (default ".")
    #[arg(short = 'p', long, default_value_t = String::from("."))]
    input_path: String,

    /// An optional string to append to layer and function keys to use as a version indicator
    #[arg(short, long)]
    version_suffix: Option<String>,

    /// An optional path within the zip to save the files to
    #[arg(long)]
    root_dir: Option<String>,
}

/// Zips up function assets and uploads them to Google Cloud Storage for use in Cloud Functions.
pub async fn push_gcs(args: GCPArgs) {
  for bucket in args.buckets.iter() {
    let file_list = search(&args.input_path, &args.include, &args.exclude);
    let buffer = create_zip(&args.input_path, file_list);
    cs_upload(bucket, &args.function_key, buffer).await;
  }
}
