use crate::Result;
use reqwest::header::{
    HeaderMap,
    HeaderName,
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
///
/// # Headers Included:
/// - `Connection`: keep-alive
/// - `Accept`: application/json, text/plain, */*
/// - `x-nba-stats-token`: true
/// - `User-Agent`: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_14_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/79.0.3945.130 Safari/537.36c
/// - `x-nba-stats-origin`: stats
/// - `Sec-Fetch-Site`: same-origin
/// - `Sec-Fetch-Mode`: cors
/// - `Referer`: https://stats.nba.com/
/// - `Accept-Encoding`: gzip, deflate, br
/// - `Accept-Language`: en-US,en;q=0.9
pub fn custom_headers() -> Result<HeaderMap> {
    let mut headers_map = HeaderMap::new();

    append_header(&mut headers_map, CONNECTION, "keep-alive")?;
    append_header(&mut headers_map, ACCEPT, "application/json, text/plain, */*")?;
    append_header(&mut headers_map, HeaderName::from_static("x-nba-stats-token"), "true")?;
    append_header(
        &mut headers_map,
        USER_AGENT,
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_14_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/79.0.3945.130 Safari/537.36c"
    )?;
    append_header(&mut headers_map, HeaderName::from_static("x-nba-stats-origin"), "stats")?;
    append_header(&mut headers_map, HeaderName::from_static("Sec-Fetch-Site"), "same-origin")?;
    append_header(&mut headers_map, HeaderName::from_static("Sec-Fetch-Mode"), "cors")?;
    append_header(&mut headers_map, REFERER, "https://stats.nba.com/")?;
    append_header(&mut headers_map, ACCEPT_ENCODING, "gzip, deflate, br")?;
    append_header(&mut headers_map, ACCEPT_LANGUAGE, "en-US,en;q=0.9")?;

    Ok(headers_map)
}

/// Appends a header to the provided `HeaderMap`.
///
/// This helper function attempts to append a header to a `HeaderMap`, converting
/// the header value from a string to a `HeaderValue`.
///
/// # Arguments
/// - `headers`: The mutable `HeaderMap` to append to.
/// - `name`: The name of the header, which can be converted into `HeaderName`.
/// - `value`: The header value as a string.
///
/// # Returns
/// - `Ok(())`: If the header is successfully appended.
/// - `Err(reqwest::header::InvalidHeaderValue)`: If the conversion of the header value fails.
fn append_header(headers: &mut HeaderMap, name: impl Into<HeaderName>, value: &str) -> Result<()> {
    headers.append(name.into(), HeaderValue::from_str(value)?);
    Ok(())
}
