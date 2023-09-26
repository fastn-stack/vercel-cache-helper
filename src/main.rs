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

    let remote_client = vercel_cache_helper::get_remote_client("".to_string(), None, "".to_string());

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
