use crate::Result;
use reqwest::header::{
    HeaderMap,
    HeaderValue,
    ACCEPT,
    ACCEPT_ENCODING,
    ACCEPT_LANGUAGE,
    CONNECTION,
    REFERER,
    USER_AGENT,
};

/// Constructs a `HeaderMap` with custom headers for HTTP requests.
///
/// This function creates and returns a `HeaderMap` populated with specific headers
/// typically used for interacting with the NBA stats API. It ensures each header is
/// appended correctly and returns a result.
///
/// # Returns
/// - `Ok(HeaderMap)`: A map containing all the custom headers.
/// - `Err(reqwest::header::InvalidHeaderValue)`: If any header value fails to convert into a `HeaderValue`.
pub fn custom_headers() -> Result<HeaderMap> {
    let mut headers = HeaderMap::new();

    headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
    headers.insert(ACCEPT, HeaderValue::from_static("application/json, text/plain, */*"));
    headers.insert("x-nba-stats-token", HeaderValue::from_static("true"));
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static(
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_14_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/79.0.3945.130 Safari/537.36"
        )
    );
    headers.insert("x-nba-stats-origin", HeaderValue::from_static("stats"));
    headers.insert("Sec-Fetch-Site", HeaderValue::from_static("same-origin"));
    headers.insert("Sec-Fetch-Mode", HeaderValue::from_static("cors"));
    headers.insert(REFERER, HeaderValue::from_static("https://stats.nba.com/"));
    headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, deflate, br"));
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));

    Ok(headers)
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::header::HeaderValue;

    #[test]
    fn test_custom_headers() {
        let headers = custom_headers().expect("Failed to create headers");

        assert_eq!(headers.get(CONNECTION), Some(&HeaderValue::from_static("keep-alive")));
        assert_eq!(
            headers.get(ACCEPT),
            Some(&HeaderValue::from_static("application/json, text/plain, */*"))
        );
        assert_eq!(
            headers.get(USER_AGENT),
            Some(
                &HeaderValue::from_static(
                    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_14_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/79.0.3945.130 Safari/537.36"
                )
            )
        );
        assert_eq!(headers.get(REFERER), Some(&HeaderValue::from_static("https://stats.nba.com/")));
        assert_eq!(
            headers.get(ACCEPT_ENCODING),
            Some(&HeaderValue::from_static("gzip, deflate, br"))
        );
        assert_eq!(headers.get(ACCEPT_LANGUAGE), Some(&HeaderValue::from_static("en-US,en;q=0.9")));

        // Custom headers
        assert_eq!(headers.get("x-nba-stats-token"), Some(&HeaderValue::from_static("true")));
        assert_eq!(headers.get("x-nba-stats-origin"), Some(&HeaderValue::from_static("stats")));
        assert_eq!(headers.get("sec-fetch-site"), Some(&HeaderValue::from_static("same-origin")));
        assert_eq!(headers.get("sec-fetch-mode"), Some(&HeaderValue::from_static("cors")));
    }
}
