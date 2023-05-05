use clap::Args;

use crate::{search::search, zip::create_zip, upload::aws_s3::s3_upload};

#[derive(Args)]
pub struct AWSArgs {
    /// An array of globs defining what to bundle
    #[arg(short, long, default_values_t = [String::from("**")])]
    include: Vec<String>,
    
    /// An array of globs defining what not to bundle
    #[arg(short, long)]
    exclude: Vec<String>,
    
    /// A list of buckets to upload to (same order as the regions please)
    #[arg(short, long, required = true)]
    buckets: Vec<String>,

    /// A list of regions to upload the assets to
    #[arg(short, long, required = true)]
    regions: Vec<String>,

    /// The path/filename of the zip file in the bucket (you don't need to add the .zip extension)
    #[arg(short, long)]
    function_key: String,

    /// The path to the lambda code and node_modules (default ".")
    #[arg(short = 'p', long, default_value_t = String::from("."))]
    input_path: String,

    /// Tells the module to split out the node modules into a zip that you can create a lambda layer from
    #[arg(short, long)]
    layer_key: Option<String>,

    /// An optional string to append to layer and function keys to use as a version indicator
    #[arg(short, long)]
    version_suffix: Option<String>,

    /// An optional path within the zip to save the files to
    #[arg(long)]
    root_dir: Option<String>,

    /// Should we create a symlink from the function directory to the layer node_modules?
    #[arg(short, long, default_value_t = false)]
    symlink_node_modules: bool,
}

/// Zips up function assets and uploads them to AWS S3 for use in lambda functions.
/// Optionally creates a file for a layer as well as a file for the function itself.
pub async fn push_aws(args: AWSArgs) {
  for (ix, bucket) in args.buckets.iter().enumerate() {
    let file_list = search(&args.input_path, &args.include, &args.exclude);
    let buffer = create_zip(&args.input_path, file_list);
    s3_upload(&args.regions[ix], bucket, &args.function_key, buffer).await;
  }
}
