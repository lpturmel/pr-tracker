use std::str::FromStr;

use base64::{engine::general_purpose, Engine as _};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DevOpsError {
    #[error("Invalid repository name")]
    InvalidRepo,

    #[error("Invalid Project ID")]
    InvalidProject,

    #[error("Invalid Organization ID")]
    InvalidOrg,
}

impl FromStr for DevOpsError {
    type Err = DevOpsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GitRepositoryNotFoundException" => Ok(DevOpsError::InvalidRepo),
            "ProjectDoesNotExistWithNameException" => Ok(DevOpsError::InvalidProject),
            _ => Ok(DevOpsError::InvalidOrg),
        }
    }
}

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
    pub async fn get_prs(
        &self,
        org: &str,
        project: &str,
        repo: &str,
    ) -> Result<AzdoResponse<PrItem>, crate::error::Error> {
        let resp = self
            .inner
            .get(&format!(
                "https://dev.azure.com/{}/{}/_apis/git/repositories/{}/pullrequests?api-version=7.1",
                org, project, repo
            ))
            .send()
            .await?;

        match resp.status() {
            StatusCode::UNAUTHORIZED => Err(crate::error::Error::Unauthorized),
            StatusCode::NOT_FOUND => {
                let err = resp.json::<AzdoError>().await?;
                Err(crate::error::Error::DevOps(
                    err.type_key.parse::<DevOpsError>()?,
                ))
            }
            StatusCode::OK => {
                let resp = resp.json::<AzdoResponse<PrItem>>().await?;
                Ok(resp)
            }
            _ => {
                println!("{:?}", resp);
                Err(crate::error::Error::Unauthorized)
            }
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AzdoResponse<T> {
    pub count: u32,
    pub value: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrItem {
    pub creation_date: String,
    pub status: String,
    pub title: String,
    pub description: Option<String>,
    pub reviewers: Vec<Reviewer>,
    pub repository: Repository,
    pub pull_request_id: u32,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reviewer {
    pub reviewer_url: String,
    pub id: String,
    pub display_name: String,
    pub unique_name: String,
    pub url: String,
    pub image_url: String,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    pub id: String,
    pub name: String,
    pub url: String,
    pub project: Project,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AzdoError {
    message: String,
    type_name: String,
    type_key: String,
    error_code: u32,
}
