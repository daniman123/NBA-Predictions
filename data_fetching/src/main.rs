use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CONNECTION, REFERER, USER_AGENT};

fn main() {
    let mut headers_map = HeaderMap::new();
    headers_map.append(CONNECTION,HeaderValue::from_str("keep-alive").unwrap());
    headers_map.append(ACCEPT,HeaderValue::from_str("application/json, text/plain, */*").unwrap());
    // headers_map.append(HeaderValue::from_str(,"keep-alive").unwrap());
    headers_map.append(USER_AGENT,HeaderValue::from_str("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_14_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/79.0.3945.130 Safari/537.36").unwrap());
    // headers_map.append(HeaderValue::from_str(SEC,"keep-alive").unwrap());
    headers_map.append(REFERER,HeaderValue::from_str("https://stats.nba.com/").unwrap());
    headers_map.append(ACCEPT_ENCODING,HeaderValue::from_str("gzip, deflate, br").unwrap());
    headers_map.append(ACCEPT_LANGUAGE,HeaderValue::from_str("en-US,en;q=0.9").unwrap());


    // let headers = RequestBuilder::headers(headers_MAP);
}



// 'Connection': 'keep-alive',
// 'Accept': 'application/json, text/plain, */*',
// 'x-nba-stats-token': 'true',
// 'User-Agent': 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_14_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/79.0.3945.130 Safari/537.36',
// 'x-nba-stats-origin': 'stats',
// 'Sec-Fetch-Site': 'same-origin',
// 'Sec-Fetch-Mode': 'cors',
// 'Referer': 'https://stats.nba.com/',
// 'Accept-Encoding': 'gzip, deflate, br',
// 'Accept-Language': 'en-US,en;q=0.9'