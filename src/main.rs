//! This crate provides a CLI interface to interface to Amber Energy's REST API.
//! Amber energy is an Australia power retailer.
//! This tool will let you query the available endpoint's offered.
//! Fetching the data and returning it as a JSON encoded dataset.
//!
//! [Amber Energy]: https://www.amber.com.au
//! [Amber API documentation]: https://app.amber.com.au/developers/documentation

use anyhow::{Ok, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use amber_client::app_config::AppConfig;
use amber_client::{
    get_prices, get_renewables, get_site_data, get_usage_by_date, get_user_site_id,
};

// Main CLI options
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    config_file: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

/// Commands to interact with the Amber API.
#[derive(Subcommand, Debug)]
enum Commands {
    SiteDetails,
    #[command(subcommand)]
    CurrentPrice(Window),
    #[command(subcommand)]
    Usage(Dates),
    #[command(subcommand)]
    Renewables(Window),
}

/// Price window to query for data.(current, next, previous)
#[derive(Parser, Debug)]
enum Window {
    /// Current interval pricing estimate.
    Current,
    /// Actual interval pricing.
    Previous,
    /// Forecast interval pricing.
    Next,
}
/// Date range to query history data for. (Using: yyyy-mm-dd format)
#[derive(Clone, Debug, Subcommand)]
enum Dates {
    DateRange {
        /// Start date to query from.
        start_date: String,
        /// End date of query from.
        end_date: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // parse cli input
    let cli_args = Cli::parse();

    // map the CLI argument of "config_file: PathBuf" to a string, using lossy conversion
    // making this is safe to use with non uni-code data.
    // https://doc.rust-lang.org/std/path/struct.Path.html#method.display
    let app_config_file = cli_args.config_file.display().to_string();

    // read config file
    let config = AppConfig::get(app_config_file).await?;

    // map API token, Amber url and users state from config
    let auth_token = config.apitoken.psk;
    let base_url = config.amberconfig.base_url;
    let users_state = config.userconfig.state;

    // Get the Site ID first, so that we can reuse it later without an additonal API call.
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

        Commands::Renewables(Window::Current) => {
            let _window = "current".to_string();
            let renewables_percent_in_grid_data =
                get_renewables(base_url, auth_token, users_state, _window).await?;
            let renewables_percent_in_grid_json =
                serde_json::to_string(&renewables_percent_in_grid_data)?;
            println!("{}", renewables_percent_in_grid_json);
        }

        Commands::Renewables(Window::Previous) => {
            let _window = "current?previous=1".to_string();
            let renewables_percent_in_grid_data =
                get_renewables(base_url, auth_token, users_state, _window).await?;
            let renewables_percent_in_grid_json =
                serde_json::to_string(&renewables_percent_in_grid_data)?;
            println!("{}", renewables_percent_in_grid_json);
        }

        Commands::Renewables(Window::Next) => {
            let _window = "current?next=1".to_string();
            let renewables_percent_in_grid_data =
                get_renewables(base_url, auth_token, users_state, _window).await?;
            let renewables_percent_in_grid_json =
                serde_json::to_string(&renewables_percent_in_grid_data)?;
            println!("{}", renewables_percent_in_grid_json);
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
