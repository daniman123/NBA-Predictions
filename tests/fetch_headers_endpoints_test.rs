use construct_api_endpoints::build_game_log_endpoint;
use data_fetching::utils::*;
use fetch_request::fetch;
use req_headers::custom_headers;

#[tokio::test]
async fn test_fetch_with_headers_constructed_headers() {
    // define headers
    let headers = custom_headers();
    // construct endpoint
    let url = build_game_log_endpoint("T", "2023", "Regular Season");
    // fetch
    let result = fetch(url.unwrap(), Some(headers.unwrap())).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(response.status().is_success());
}
