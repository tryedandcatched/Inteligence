use std::{collections::HashMap, time::Duration};

pub fn get_stealh_client() -> reqwest::Client {
    let mut headers_h: HashMap<reqwest::header::HeaderName, reqwest::header::HeaderValue> =
        HashMap::new();

    headers_h.insert(reqwest::header::USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36".parse().unwrap());
    headers_h.insert(
        reqwest::header::ACCEPT,
        "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8"
            .parse()
            .unwrap(),
    );
    headers_h.insert(
        reqwest::header::ACCEPT_LANGUAGE,
        "en-US,en;q=0.5".parse().unwrap(),
    );
    headers_h.insert(reqwest::header::ACCEPT_ENCODING, "deflate".parse().unwrap());
    headers_h.insert(reqwest::header::CONNECTION, "keep-alive".parse().unwrap());
    headers_h.insert(
        reqwest::header::REFERER,
        "https://google.com".parse().unwrap(),
    );

    let mut headers = reqwest::header::HeaderMap::new();
    for (key, value) in headers_h {
        headers.insert(key, value);
    }

    reqwest::ClientBuilder::new()
        .timeout(Duration::new(5, 0))
        .default_headers(headers)
        .build()
        .unwrap()
}
