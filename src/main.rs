use std::path::PathBuf;

use anyhow::{Ok, Result};
use clap::{Parser, Subcommand};

use amber_client::app_config::AppConfig;
use amber_client::{get_current_prices, get_site_data};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    config_file: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    SiteDetails,
    CurrentPrice,
    Usage,
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
        Commands::CurrentPrice => {
            let current_price_data = get_current_prices(base_url, auth_token, site_id).await?;
            let current_price_data_json = serde_json::to_string(&current_price_data)?;
            println!("{}", current_price_data_json);
        }
        Commands::SiteDetails => {
           let site_data = get_site_data(base_url, auth_token).await?;
           let site_data_json = serde_json::to_string(&site_data)?;
           println!("{}", site_data_json);
        }

        Commands::Usage => {
            println!("not done yet");
        }
    }

    async fn get_user_site_id(base_url: String, auth_token: String) -> Result<String> {
        let user_site_data = get_site_data(base_url, auth_token).await?;
        let user_site_id = user_site_data.id;
        Ok(user_site_id)
    }

    Ok(())
}
