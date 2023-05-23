#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![doc(issue_tracker_base_url = "https://github.com/bbeesley/push-fns/issues/")]

/// The CLI arguments
pub mod args;

/// Functions for uploading to AWS S3
pub mod aws;
/// Functions for uploading to GCP Cloud Storage
pub mod gcp;
/// Functions for searching the filesystem based on include and exclude globs
pub mod search;
/// Generic upload functions for S3 and GCS
pub mod upload;
/// Functions for adding a list of files to a zip archive
pub mod zip;

use aws::push_aws;
use clap::Parser;
use gcp::push_gcs;

use crate::args::{Cli, Commands};

/// The entrypoint for the CLI - parses the CLI args and calls the appropriate function
#[tokio::main]
async fn main() {
  let cli = args::Cli::parse();
  match cli {
    Cli {
      command: Commands::Aws(args),
    } => {
      push_aws(args).await;
    }
    Cli {
      command: Commands::Gcp(args),
    } => {
      push_gcs(args).await;
    }
  }
}
