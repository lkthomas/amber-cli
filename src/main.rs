use anyhow::Result;

use amber_client::app_config::AppConfig;
use amber_client::rest_client::RestClient;

#[tokio::main]
async fn main() -> Result<()> {
    // get config
    let config = AppConfig::get().await?;
    let auth_token = config.apitoken.psk;
    let base_url = config.amberconfig.base_url;

    // get site details
    let sites_url = format!("{}/sites", base_url);
    let mut user_site_details = RestClient::new_client(sites_url, auth_token.clone());
    let user_site_data = user_site_details.get_site_data().await?;

    // one account can only have one site, so extract from array
    let user_site_data = user_site_data
        .get(0)
        .expect("Malformed array/invalid index[0]");

    let site_id = &user_site_data.id;

    // end site details

    // get current price details
    let current_price_url = format!(
        "{}/sites/{}/prices/current?&resolution=30",
        base_url, site_id
    );
    let mut current_price_details = RestClient::new_client(current_price_url, auth_token.clone());
    let current_price_data = current_price_details.get_current_price_data().await?;

    // One site can only have one set of current prices so extract from array
    let current_price_data = current_price_data
        .get(0)
        .expect("Malformed array/invalid index[0]");

    // end current price details

    // get usage dat
    let usage_data_url = format!(
        "{}/sites/{}/usage?startDate=2023-09-12&endDate=2023-09-13&resolution=30'",
        base_url, site_id
    );
    let mut usage_details = RestClient::new_client(usage_data_url, auth_token.clone());
    let usage_data = usage_details.get_usage_data().await?;

    // end usage data

    println!("-------------------------------------------------------------------");
    println!("My site details");
    println!("Grid network: {}", &user_site_data.network);
    println!("My house meter NMI number: {}", &user_site_data.nmi);
    println!("Status: {}", &user_site_data.status);
    println!("-------------------------------------------------------------------");
    println!("Current 30min price window rate");
    println!("Window stats at: {}", &current_price_data.start_time);
    println!("Window ends at: {}", &current_price_data.end_time);
    println!("Per KWH price(c/kWh): {}", &current_price_data.per_kwh);
    println!(
        "Is this window in a spike?: {}",
        &current_price_data.spike_status
    );
    println!("Overall rate status: {}", &current_price_data.descriptor);
    println!("-------------------------------------------------------------------");
    //println!("{:#?}", usage_data);

    Ok(())
}
