use base64::{engine::general_purpose, Engine as _};
#[derive(Debug)]
pub struct Client {
    inner: reqwest::Client,
}

impl Client {
    pub fn new(pat: &str) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        let user = format!(":{}", pat);
        headers.insert(
            reqwest::header::AUTHORIZATION,
            format!(
                "Basic {}",
                general_purpose::STANDARD_NO_PAD.encode(user.as_bytes())
            )
            .parse()
            .unwrap(),
        );
        Self {
            inner: reqwest::Client::builder()
                .default_headers(headers)
                .build()
                .unwrap(),
        }
    }
}
