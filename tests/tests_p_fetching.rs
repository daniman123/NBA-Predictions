use data_fetching::utils::fetch_request::fetch;

#[cfg(test)]
mod tests {

    use serde_json::Value;

    use super::*;

    #[tokio::test]
    async fn test_name() {
        let url = "https://www.rotowire.com/basketball/tables/injury-report.php?team=ALL&pos=ALL";
        let result = fetch(url, None).await;
        assert!(result.is_ok());
        let response = result.unwrap();
        println!("{:?}", response);
        assert!(response.status().is_success());
        let response_data_result = response.json::<Value>().await;
        assert!(response_data_result.is_ok());
        let response_data_json = response_data_result.unwrap();
        println!("response_data_json: {:?}", response_data_json);
    }
}
