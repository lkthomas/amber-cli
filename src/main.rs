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

use tracing::{debug, Instrument};
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::{prelude::*, EnvFilter};

use amber_client::app_config::AppConfig;
use amber_client::{
    get_prices, get_renewables, get_site_data, get_usage_by_date, get_user_site_id,
    write_data_as_csv_to_file,
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
    Price(Window),
    #[command(subcommand)]
    Usage(Dates),
    #[command(subcommand)]
    Renewables(Window),
}

/// Price window to query for data.(current, next, previous)
#[derive(Parser, Debug)]
enum Window {
    /// Current interval data.
    Current,
    /// Previous interval data.
    Previous,
    /// Forecast interval data.
    Next,
}
/// Date range to query history data for. (Using: yyyy-mm-dd format)
// Not super keen on the way this is structured, but works for now.
// Would like the export options to be more obvious.
#[derive(Clone, Debug, Subcommand)]
enum Dates {
    DateRange {
        /// Start date to query from.
        start_date: String,
        /// End date of query from.
        end_date: String,
        /// [Optional] Path to save/export data in CSV format.
        filename_to_export_to: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Set up a default layer for formating trace/log messages
    let default_layer_format = tracing_subscriber::fmt::layer()
        .compact()
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .with_target(false);

    // Configure tracing registry and enable tracing
    tracing_subscriber::registry()
        .with(default_layer_format)
        // Read environment variable "RUST_LOG" to determine log level
        .with(EnvFilter::from_default_env())
        .try_init()?;

    // parse cli input
    let cli_args = Cli::parse();

    // map the CLI argument of "config_file: PathBuf" to a string, using lossy conversion
    // making this is safe to use with non uni-code data.
    // https://doc.rust-lang.org/std/path/struct.Path.html#method.display
    let app_config_file = cli_args.config_file.display().to_string();

    // read config file
    //let load_app_config = debug_span!("Loading app config");
    debug!("Loaded config file: {}", app_config_file.clone());
    let config = AppConfig::get(app_config_file)
        .instrument(tracing::debug_span!("some_other_async_function"))
        .await?;

    // map API token, Amber url and users state from config
    let auth_token = config.apitoken.psk;
    let base_url = config.amberconfig.base_url;
    let users_state = config.userconfig.state;

    // Get the Site ID first, so that we can reuse it later without an additonal API call.
    let site_id = get_user_site_id(base_url.clone(), auth_token.clone()).await?;

    match cli_args.command {
        Commands::Price(Window::Current) => {
            let _window = "current".to_string();
            let current_price_data = get_prices(base_url, auth_token, site_id, _window).await?;
            let current_price_data_json = serde_json::to_string(&current_price_data)?;
            println!("{}", current_price_data_json);
        }
        Commands::Price(Window::Previous) => {
            let _window = "current?previous=1".to_string();
            let current_price_data = get_prices(base_url, auth_token, site_id, _window).await?;
            let current_price_data_json = serde_json::to_string(&current_price_data)?;
            println!("{}", current_price_data_json);
        }

        Commands::Price(Window::Next) => {
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
            filename_to_export_to,
        }) => {
            let usage =
                get_usage_by_date(base_url, auth_token, site_id, start_date, end_date).await?;

            // If the Option<path> contains a value then we enter export/save to file mode.
            match filename_to_export_to {
                Some(filename) => {
                    let filename_as_string = filename.display().to_string();
                    write_data_as_csv_to_file(filename_as_string, usage.clone()).await?;
                    //println!("file: {:?}", filename);
                }
                None => {
                    let usage_json = serde_json::to_string(&usage)?;
                    println!("{}", usage_json);
                }
            }
        }
    }

    Ok(())
}
