use amber_client::rest_client::RestClient;
use amber_client::parse_date_naive;

use wiremock::matchers::{header, method};
use wiremock::{Mock, MockServer, ResponseTemplate};

mod mock_data {
    use amber_client::rest_client::SiteChannels;
    use amber_client::rest_client::SiteDetails;
    use iso8601_timestamp::Timestamp;

    pub fn site_details_json_struct() -> Vec<SiteDetails> {
        let test_time_date_stamp = "2023-08-31T00:00:00.000Z";
        let test_timestamp = Timestamp::parse(test_time_date_stamp).unwrap();

        let site_data = vec![SiteDetails {
            id: "test_site_id".to_string(),
            nmi: "1234567890".to_string(),
            channels: vec![SiteChannels {
                identifier: "E1".to_string(),
                tariff_type: "general".to_string(),
                tariff: "A123".to_string(),
            }],
            network: "test_network".to_string(),
            status: "active".to_string(),
            active_from: test_timestamp,
        }];
        site_data
    }


    pub fn amber_site_details_json() -> String {
        let new_amber_site_details_json = r#"[
          {
            "activeFrom": "2023-08-31T00:00:00.000Z",
            "channels": [
              {
                "identifier": "E1",
                "tariff": "A123",
                "type": "general"
              }
            ],
            "id": "test_site_id",
            "network": "test_network",
            "nmi": "1234567890",
            "status": "active"
          }
       ]"#
        .to_string();
        return new_amber_site_details_json;
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
async fn valid_json_parsing_for_site_details() {
    let mock_server = MockServer::start().await;
    let template = ResponseTemplate::new(200)
        .set_body_raw(mock_data::amber_site_details_json(), "application/json");
    let mut user_site_details = RestClient::new_client(mock_server.uri(), "token".to_string());

    Mock::given(method("GET"))
        .respond_with(template)
        .mount(&mock_server)
        .await;

    let test_site_details_request = user_site_details.get_site_data().await.unwrap();

    assert_eq!(test_site_details_request, mock_data::site_details_json_struct());
}
#[tokio::test]
// use should_panic to capture the following:
// thread 'unauthorized_api_access' panicked at 'called `Result::unwrap()` on an `Err` value: HttpNon200Status
// { status_code: "401 Unauthorized", body: "{\"message\": \"Unauthorized\"}" }',
#[should_panic(expected = "401 Unauthorized")]
async fn unauthorized_api_access() {
    let mock_server = MockServer::start().await;
    let template = ResponseTemplate::new(401)
        .set_body_raw(mock_data::amber_401_unauthorized(), "application/json");
    let mut unauthorized_access = RestClient::new_client(mock_server.uri(), "token".to_string());

    Mock::given(method("GET"))
        .respond_with(template)
        .mount(&mock_server)
        .await;

    let _test_site_details_request = unauthorized_access.get_site_data().await.unwrap();
}

#[tokio::test]
async fn date_validator_parser_valid_date() {

    let valid_date_string = "2023-12-31".to_string();
    let test_date_result = parse_date_naive(valid_date_string).await.unwrap();

    assert_eq!(test_date_result, "2023-12-31".to_string());

}