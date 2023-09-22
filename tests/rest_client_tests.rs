use amber_client::rest_client::RestClient;

use wiremock::matchers::{header, method};
use wiremock::{Mock, MockServer, ResponseTemplate};

mod mock_data {
    use amber_client::rest_client::SiteChannels;
    use amber_client::rest_client::SiteDetails;
    use iso8601_timestamp::Timestamp;

    pub fn site_details_json_struct() -> SiteDetails {
        let test_time_date_stamp = "2021-05-05T00:00:00Z";
        let test_timestamp = Timestamp::parse(test_time_date_stamp).unwrap();

        let site_data = SiteDetails {
            id: "516659425499187395570254629".to_string(),
            nmi: "50147919623".to_string(),
            channels: vec![SiteChannels {
                identifier: "E1".to_string(),
                tariff_type: "general".to_string(),
                tariff: "A100".to_string(),
            }],
            network: "test".to_string(),
            status: "testing".to_string(),
            active_from: test_timestamp,
        };
        site_data
    }

    pub fn amber_site_details_json() -> String {
        r#"[{"id": "516659425499187395570254629", "nmi": "50147919623", 
        "channels": [{"identifier": "E1", "type": "general", "tariff": "A100"}], "network": "test", 
        "status": "testing", "activeFrom": "2021-05-05", "closedOn": "2022-05-01"}]"#
            .to_string()
    }

    pub fn amber_401_unauthorized() -> String {
        r#"{"message": "Unauthorized"}"#.to_string()
    }
}

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

#[tokio::test]
async fn test_valid_json_parsing_for_site_details() {
    let mock_server = MockServer::start().await;
    let template = ResponseTemplate::new(200)
        .set_body_raw(mock_data::amber_site_details_json(), "application/json");
    let mut user_site_details = RestClient::new_client(mock_server.uri(), "token".to_string());

    Mock::given(method("GET"))
        .respond_with(template)
        .mount(&mock_server)
        .await;

    let test_site_details_request = user_site_details.get_site_data().await.unwrap();

    let test_user_site_data = test_site_details_request
        .get(0)
        .expect("Malformed array/invalid index[0]");

    assert_eq!(test_user_site_data, &mock_data::site_details_json_struct());
}
#[tokio::test]
async fn test_unauthorized_api_access() {
    let mock_server = MockServer::start().await;
    let template = ResponseTemplate::new(200)
        .set_body_raw(mock_data::amber_401_unauthorized(), "application/json");
    let mut unauthorized_access = RestClient::new_client(mock_server.uri(), "token".to_string());

    Mock::given(method("GET"))
        .respond_with(template)
        .mount(&mock_server)
        .await;

    let unauthorized_access_request = unauthorized_access.get_site_data().await.unwrap();

    println!("{:#?}", unauthorized_access_request);
}
