use construct_api_endpoints::build_game_log_endpoint;
use data_fetching::utils::*;
use fetch_request::fetch;
use req_headers::custom_headers;
use data_fetching::utils::process_bytes::extract_bytes_from_response;
use data_fetching::utils::read_write_from_file_tools::write_to_file;

#[tokio::test]
async fn test_fetch_with_headers_constructed_headers() {
    // define headers
    let headers = custom_headers();
    // construct endpoint
    let url = build_game_log_endpoint("T", "2023", "Regular Season");
    println!("{:?}", url);
    // fetch
    let result = fetch(url.unwrap(), Some(headers.unwrap())).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    println!("{:?}", response);
    assert!(response.status().is_success());

    let raw_bytes_data = extract_bytes_from_response(Ok(response)).await;
    assert!(raw_bytes_data.is_ok());

    let path = "../data/raw_response_data/test_output.bin";
    let write_result = write_to_file(path, raw_bytes_data.unwrap());

    assert!(write_result.is_ok());
}
