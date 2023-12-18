use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

use amber_client::app_config::AppConfig;
use amber_client::get_site_data;

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

    // get site details

    // let user_site_data = get_site_data();
    let user_site_data = get_site_data(base_url, auth_token).await?;

    let site_id = user_site_data.id;

    println!("{:?}", site_id);

    Ok(())
}
