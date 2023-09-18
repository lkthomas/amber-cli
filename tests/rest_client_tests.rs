use amber_client::rest_client::RestClient;

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
    pub fn body() -> Vec<u8> {
        r#"{"id": "01F5A5CRKMZ5BCX9P1S4V990AM", "nmi": "3052282872", "channels": [{"identifier": "E1", "type": "general", "tariff": "A100"}], "network": "Jemena", "status": "active", "activeFrom": "2022-01-01", "closedOn": "2022-05-01"}"#.as_bytes().to_owned()
    }
}

#[tokio::test]
async fn check_site_data_responce() {
    let mock_server = MockServer::start().await;
    let template =
        ResponseTemplate::new(200).set_body_raw(external_site_data::body(), "application/json");
    let mut user_site_details = RestClient::new_client(mock_server.uri(), "token".to_string());

    Mock::given(method("GET"))
        .respond_with(template)
        .mount(&mock_server)
        .await;

let brrr = user_site_details.get_site_data().await.unwrap();

assert_eq!(brrr,  external_site_data::body());


//    let mut res = surf::get(&mock_server.uri()).await.unwrap();
    //let body = brrr.body_string().await.unwrap();

//    assert_eq!(
//        brrr,
 //       r#"{"id": "01F5A5CRKMZ5BCX9P1S4V990AM", "nmi": "3052282872", "channels": [{"identifier": "E1", "type": "general", "tariff": "A100"}], "network": "Jemena", "status": "active", "activeFrom": "2022-01-01", "closedOn": "2022-05-01"}"#
 //   );
}
