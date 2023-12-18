use anyhow::Result;

pub mod app_config;
pub mod rest_client;

use rest_client::{CurrentPrices, RestClient, SiteDetails};

// get site details
pub async fn get_site_data(base_url: String, auth_token: String) -> Result<SiteDetails> {
    let sites_url = format!("{}/sites", base_url);
    let mut user_site_details = RestClient::new_client(sites_url, auth_token.clone());
    let user_site_data = user_site_details.get_site_data().await?;

    // one account can only have one site, so extract from array
    let user_site_data = user_site_data
        .get(0)
        .expect("Malformed array/invalid index[0]");

    let single_site_data = user_site_data.clone();
    Ok(single_site_data)
}

// get current price rates
pub async fn get_current_prices(
    base_url: String,
    auth_token: String,
    site_id: String,
) -> Result<CurrentPrices> {
    let current_price_url = format!(
        "{}/sites/{}/prices/current?&resolution=30",
        base_url, site_id
    );
    let mut current_price_details = RestClient::new_client(current_price_url, auth_token.clone());
    let current_price_data = current_price_details.get_current_price_data().await?;

    // One site can only have one set of current prices so extract from arrayw
    let current_price_data = current_price_data
        .get(0)
        .expect("Malformed array/invalid index[0]");

    let single_site_price_data = current_price_data.clone();

    Ok(single_site_price_data)
}

// not done yet, need to feed it a date
// pub async fn get_usage_by_date(base_url: String, auth_token: String, site_id: String) {
// let usage_data_url = format!(
//    "{}/sites/{}/usage?startDate=2023-09-12&endDate=2023-09-13&resolution=30'",
//    base_url, site_id
// );
// let mut usage_details = RestClient::new_client(usage_data_url, auth_token.clone());
// let usage_data = usage_details.get_usage_data().await?;
// Ok(())
// }
