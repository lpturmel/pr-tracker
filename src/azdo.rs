use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Client {
    inner: reqwest::Client,
}

impl Client {
    pub fn new(pat: &str) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        let user = format!(":{}", pat);
        let encoded = general_purpose::STANDARD.encode(user.as_bytes());
        headers.insert(
            reqwest::header::AUTHORIZATION,
            format!("Basic {}", encoded).parse().unwrap(),
        );
        Self {
            inner: reqwest::Client::builder()
                .default_headers(headers)
                .build()
                .unwrap(),
        }
    }

    pub async fn get_profile(&self) -> Result<ConnectionData, reqwest::Error> {
        let resp = self
            .inner
            .get("https://dev.azure.com/2020Development/_apis/ConnectionData")
            .send()
            .await?;
        resp.json::<ConnectionData>().await
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionData {
    pub authenticated_user: AuthenticatedUser,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticatedUser {
    pub id: String,
    pub provider_display_name: String,
    pub properties: UserProperties,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserProperties {
    #[serde(rename = "Account")]
    pub account: UserAccount,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserAccount {
    #[serde(rename = "$type")]
    pub user_type: String,

    #[serde(rename = "$value")]
    pub value: String,
}
