use reqwest::header::*;
use serde::Serialize;
use std::env;
use std::time::Duration;

#[derive(Debug, Serialize, PartialEq)]
pub struct Footer {
    pub(crate) text: String,
}

#[derive(Debug, Default, Serialize, PartialEq)]
pub struct Provider {
    pub(crate) name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) url: Option<String>,
}

#[derive(Debug, Default, Serialize, PartialEq)]
pub struct Field {
    pub(crate) name: String,
    pub(crate) value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) inline: Option<bool>,
}

#[derive(Debug, Default, Serialize, PartialEq)]
pub struct Embed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) footer: Option<Footer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) provider: Option<Provider>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) fields: Option<Vec<Field>>,
}

#[derive(Debug, Default, Serialize, PartialEq)]
pub struct Webhook {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) avatar_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) embeds: Option<Vec<Embed>>,
}

pub async fn send(content: Webhook) {
    let client = reqwest::Client::new();
    let mut header = HeaderMap::new();
    header.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    let body = serde_json::to_string(&content).unwrap();
    println!("{}", body);
    let url = env::var("WEBHOOK").expect("WEBHOOK must be set");
    loop {
        let response = client
            .post(&url)
            .headers(header.clone())
            .query(&[("wait", "true")])
            .body(body.clone())
            .send()
            .await
            .unwrap();
        if response.status() == 200 {
            break;
        } else if response.status() == 429 {
            tokio::time::sleep(Duration::from_secs(
                response.headers()["Retry-After"]
                    .to_str()
                    .unwrap()
                    .parse::<u64>()
                    .unwrap(),
            ))
            .await;
            continue;
        } else {
            println!("{}", response.text().await.unwrap());
            break;
        }
    }
}
