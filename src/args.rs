use clap::{Args, Parser, Subcommand};

/// A simple tool to upload serverless function assets
#[derive(Parser, Debug)]
#[command(name = "push-fns", author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
  /// The commands for the CLI
  #[command(subcommand)]
  pub command: Commands,
}

/// The subcommands for the CLI
#[derive(Subcommand, Debug)]
pub enum Commands {
  /// Zips up function assets and uploads them to AWS S3 for use in lambda functions.
  /// Optionally creates a file for a layer as well as a file for the function itself.
  Aws(AWSArgs),

  /// Zips up function assets and uploads them to Google Cloud Storage for use in Cloud Functions.
  Gcp(GCPArgs),
}

/// The arguments for the GCP upload function
#[derive(Args, Debug)]
pub struct GCPArgs {
  /// An array of globs defining what to bundle
  #[arg(short, long, default_values_t = [String::from("**")])]
  pub include: Vec<String>,

  /// An array of globs defining what not to bundle
  #[arg(short, long)]
  pub exclude: Vec<String>,

  /// A list of buckets to upload to (same order as the regions please)
  #[arg(short, long, required = true)]
  pub buckets: Vec<String>,

  /// The path/filename of the zip file in the bucket (you don't need to add the .zip extension)
  #[arg(short, long)]
  pub function_key: String,

  /// The path to the lambda code and node_modules (default ".")
  #[arg(short = 'p', long, default_value_t = String::from("."))]
  pub input_path: String,

  /// An optional string to append to layer and function keys to use as a version indicator
  #[arg(short, long)]
  pub version_suffix: Option<String>,

  /// An optional path within the zip to save the files to
  #[arg(long)]
  pub root_dir: Option<String>,
}

/// The arguments for the AWS upload function
#[derive(Args, Debug)]
pub struct AWSArgs {
  /// An array of globs defining what to bundle
  #[arg(short, long, default_values_t = [String::from("**")])]
  pub include: Vec<String>,

  /// An array of globs defining what not to bundle
  #[arg(short, long)]
  pub exclude: Vec<String>,

  /// A list of buckets to upload to (same order as the regions please)
  #[arg(short, long, required = true)]
  pub buckets: Vec<String>,

  /// A list of regions to upload the assets to
  #[arg(short, long, required = true)]
  pub regions: Vec<String>,

  /// The path/filename of the zip file in the bucket (you don't need to add the .zip extension)
  #[arg(short, long)]
  pub function_key: String,

  /// The path to the lambda code and node_modules (default ".")
  #[arg(short = 'p', long, default_value_t = String::from("."))]
  pub input_path: String,

  /// Tells the module to split out the node modules into a zip that you can create a lambda layer from
  #[arg(short, long)]
  pub layer_key: Option<String>,

  /// An array of globs defining what to include in the layer zip
  #[arg(long, default_values_t = [String::from("node_modules/**")])]
  pub layer_globs: Vec<String>,

  /// An optional string to append to layer and function keys to use as a version indicator
  #[arg(short, long)]
  pub version_suffix: Option<String>,

  /// An optional path within the zip to save the files to
  #[arg(long)]
  pub root_dir: Option<String>,

  /// Should we create a symlink from the function directory to the layer node_modules?
  #[arg(short, long, default_value_t = false)]
  pub symlink_node_modules: bool,
}
