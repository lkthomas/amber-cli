use std::path::PathBuf;

use anyhow::{Ok, Result};
use clap::{Parser, Subcommand};

use amber_client::app_config::AppConfig;
use amber_client::{get_prices, get_site_data, get_usage_by_date, get_user_site_id};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    config_file: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    SiteDetails,
    #[command(subcommand)]
    CurrentPrice(Window),
    #[command(subcommand)]
    Usage(Dates),
}

#[derive(Parser, Debug)]
enum Window {
    Current,
    Previous,
    Next,
}

// #[derive(Parser, Debug)]
#[derive(Clone, Debug, Subcommand)]
enum Dates {
    DateRange {
        start_date: String,
        end_date: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // parse cli input
    let cli_args = Cli::parse();

    // map CLI PathBuf to string, lossy conversion
    // https://doc.rust-lang.org/std/path/struct.Path.html#method.display
    let app_config_file = cli_args.config_file.display().to_string();

    // get app config from file
    let config = AppConfig::get(app_config_file).await?;

    // map ap token and url from config
    let auth_token = config.apitoken.psk;
    let base_url = config.amberconfig.base_url;

    // set up Site ID, reduce API calls and the API only supports one site ID per user.
    let site_id = get_user_site_id(base_url.clone(), auth_token.clone()).await?;

    match cli_args.command {
        Commands::CurrentPrice(Window::Current) => {
            let _window = "current".to_string();
            let current_price_data = get_prices(base_url, auth_token, site_id, _window).await?;
            let current_price_data_json = serde_json::to_string(&current_price_data)?;
            println!("{}", current_price_data_json);
        }
        Commands::CurrentPrice(Window::Previous) => {
            let _window = "current?previous=1".to_string();
            let current_price_data = get_prices(base_url, auth_token, site_id, _window).await?;
            let current_price_data_json = serde_json::to_string(&current_price_data)?;
            println!("{}", current_price_data_json);
        }

        Commands::CurrentPrice(Window::Next) => {
            let _window = "current?next=1".to_string();
            let current_price_data = get_prices(base_url, auth_token, site_id, _window).await?;
            let current_price_data_json = serde_json::to_string(&current_price_data)?;
            println!("{}", current_price_data_json);
        }

        Commands::SiteDetails => {
            let site_data = get_site_data(base_url, auth_token).await?;
            let site_data_json = serde_json::to_string(&site_data)?;
            println!("{}", site_data_json);
        }

        Commands::Usage(Dates::DateRange {
            start_date,
            end_date,
        }) => {
            let usage =
                get_usage_by_date(base_url, auth_token, site_id, start_date, end_date).await?;
            let usage_json = serde_json::to_string(&usage)?;
            println!("{}", usage_json);
        }
    }

    Ok(())
}
