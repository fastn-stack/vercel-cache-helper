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

fn main() {
    let cli = Cli::parse();

    let token = std::env::var("VERCEL_TOKEN").expect("Vercel token not found");
    let team_id = std::env::var("VERCEL_TEAM_ID").unwrap_or("".to_string());
    let product = std::env::var("VERCEL_PRODUCT").expect("Vercel product name not found");

    let remote_client = vercel_cache_helper::get_remote_client(token, Some(team_id), product);

    match &cli.command {
        Some(Commands::Download { path }) => {
            if let Some(location) = path {
                println!("{}", location.to_string_lossy());
            }
        }
        Some(Commands::Upload { path }) => { }
        None => {}
    }
}
