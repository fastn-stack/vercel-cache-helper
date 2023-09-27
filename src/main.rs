use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Download {},
    Upload {},
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let token = std::env::var("VERCEL_TOKEN").expect("Vercel token not found");
    let team_id = std::env::var("VERCEL_TEAM_ID").unwrap_or("".to_string());
    let product = std::env::var("VERCEL_PRODUCT").expect("Vercel product name not found");

    let remote_client = vercel_cache_helper::get_remote_client(token, Some(team_id), product);

    let result_future: std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), vercel_cache_helper::Error>>>> = match &cli.command {
        Some(Commands::Download {}) => {
            Box::pin(vercel_cache_helper::commands::download::download(remote_client))
        }
        Some(Commands::Upload {}) => {
            Box::pin(vercel_cache_helper::commands::upload::upload(remote_client))
        }
        None => Box::pin(async { Ok(()) }),
    };

    // Now you can await the result_future if necessary
    let _ = result_future.await;
}
