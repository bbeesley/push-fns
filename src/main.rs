use clap::{Parser, Subcommand};
use push_fns::{aws::{push_aws, AWSArgs}, gcp::{push_gcs, GCPArgs}};

/// A simple tool to upload serverless function assets
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Zips up function assets and uploads them to AWS S3 for use in lambda functions.
    /// Optionally creates a file for a layer as well as a file for the function itself.
    Aws(AWSArgs),

    /// Zips up function assets and uploads them to Google Cloud Storage for use in Cloud Functions.
    Gcp(GCPArgs),
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
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
