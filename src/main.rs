use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Download {
        #[arg(short, long)]
        path: Option<std::path::PathBuf>,
    },
    Upload {
        #[arg(short, long)]
        path: Option<std::path::PathBuf>,
    },
}

#[tokio::main]
async fn main() -> vercel_cache_helper::Result<()> {
    let cli = Cli::parse();

    let token = std::env::var("VERCEL_TOKEN").map_err(|_e| {
        vercel_cache_helper::Error::EnvVarNotFound("Vercel token not found".to_string())
    })?;
    let team_id = std::env::var("VERCEL_TEAM_ID").unwrap_or("".to_string());
    let product = std::env::var("VERCEL_PRODUCT").map_err(|_e| {
        vercel_cache_helper::Error::EnvVarNotFound("Vercel product name not found".to_string())
    })?;

    let remote_client = vercel_cache_helper::get_remote_client(token, Some(team_id), product);

    println!("🚀 fastn Vercel Cache Helper v{}", env!("CARGO_PKG_VERSION"));

    let result_future: std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<(), vercel_cache_helper::Error>>>,
    > = match &cli.command {
        Some(Commands::Download { path }) => Box::pin(
            vercel_cache_helper::commands::download::download(remote_client, path),
        ),
        Some(Commands::Upload { path }) => Box::pin(vercel_cache_helper::commands::upload::upload(
            remote_client,
            path,
        )),
        None => Box::pin(async { Ok(()) }),
    };

    // Now you can await the result_future if necessary
    let _ = result_future.await;

    Ok(())
}
