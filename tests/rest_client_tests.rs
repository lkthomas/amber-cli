use amber_client::rest_client::RestClient;

use chrono::{prelude::*, Utc};


use claim::assert_ok;
use surf::http::mime;
use wiremock::matchers::{header, header_exists, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// let sites_url = format!("{}/sites", base_url);
// let mut user_site_details = RestClient::new_client(sites_url, auth_token.clone());
// let user_site_data = user_site_details.get_site_data().await?;

#[tokio::test]
async fn ensure_correct_headers_are_present_and_get_called_once() {
    let mock_server = MockServer::start().await;

    let mut user_site_details = RestClient::new_client(mock_server.uri(), "token".to_string());

    Mock::given(header("AUTHORIZATION", "Bearer token"))
        .and(header("CONTENT_TYPE", "application/json"))
        .and(method("GET"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&mock_server)
        .await;

    let _ = user_site_details.get_site_data().await;
}

mod external_site_data {
    pub fn body() -> String{ 
        r#"[{"id": "516659425499187395570254629", "nmi": "50147919623", 
        "channels": [{"identifier": "E1", "type": "general", "tariff": "A100"}], "network": "test", 
        "status": "testing", "activeFrom": "2021-05-05", "closedOn": "2022-05-01"}]"#.to_string()
    }
}

mod mock_site_data {
    use amber_client::rest_client::SiteDetails;
    use amber_client::rest_client::SiteChannels;
    use iso8601_timestamp::Timestamp;

    pub fn json_response() -> SiteDetails {
        let test_time_date_stamp ="2021-05-05T00:00:00Z";
        let test_timestamp = Timestamp::parse(test_time_date_stamp).unwrap();


        let site_data = SiteDetails {
            id:  "516659425499187395570254629".to_string(),
            nmi: "50147919623".to_string(),
            channels: vec![SiteChannels{identifier: "E1".to_string(), tariff_type: "general".to_string(), tariff: "A100".to_string()}],
            network: "test".to_string(),
            status: "testing".to_string(),
            active_from: test_timestamp,
        };
        site_data 
    }
}

#[tokio::test]
async fn check_site_data_response() {
    let mock_server = MockServer::start().await;
    let template =
        ResponseTemplate::new(200).set_body_raw(external_site_data::body(), "application/json");
    let mut user_site_details = RestClient::new_client(mock_server.uri(), "token".to_string());

    Mock::given(method("GET"))
        .respond_with(template)
        .mount(&mock_server)
        .await;

let brrr = user_site_details.get_site_data().await.unwrap();

let user_site_data = brrr
.get(0)
.expect("Malformed array/invalid index[0]");




assert_eq!(user_site_data,  &mock_site_data::json_response());


//    let mut res = surf::get(&mock_server.uri()).await.unwrap();
    //let body = brrr.body_string().await.unwrap();

//    assert_eq!(
//        brrr,
 //       r#"{"id": "01F5A5CRKMZ5BCX9P1S4V990AM", "nmi": "3052282872", "channels": [{"identifier": "E1", "type": "general", "tariff": "A100"}], "network": "Jemena", "status": "active", "activeFrom": "2022-01-01", "closedOn": "2022-05-01"}"#
 //   );
}
