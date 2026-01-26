use std::{collections::HashMap, sync::LazyLock};

use futures::lock::Mutex;
use serde::Serialize;
use tauri::{
    Url,
    http::{HeaderMap, HeaderValue},
};
use tauri_plugin_http::reqwest::{self, Client, ClientBuilder, Request, retry::Builder};
use tokio::time::{Duration, sleep};
use tokio_tungstenite::tungstenite::client;
use tracing::{info, trace};

// pub static SUBSONIC_CLIENT: LazyLock<SubsonicClient> = LazyLock::new(|| SubsonicClient {
//     state: ConnectionState::Closed,
//     client: Client::new(),
//     base: Url::parse("http://localhost:4533").unwrap(),
// });

enum ConnectionState {
    Closed,
    Connected,
}

pub struct NavidromeClient {
    state: ConnectionState,
    client: Client,
    base: Url,
    param: String,
}

impl NavidromeClient {
    pub fn new() -> Result<Self, tauri_plugin_http::Error> {
        Ok(Self {
            state: ConnectionState::Closed,
            client: Client::new(),
            base: Url::parse("http://192.168.31.238:4533/").unwrap(),
            param: String::from("?u=Looouiiis&p=Efzxlyc020722&v=1.12.0&c=apple_drome&f=json"),
        })
    }

    fn build_subsonic_url<'a>(&self, rest_str: &'a str) -> Url {
        self.base
            .join("rest")
            .unwrap()
            .join(rest_str)
            .unwrap()
            .join(self.param.as_str())
            .unwrap()
    }

    pub async fn auth(&mut self) -> Result<(), tauri_plugin_http::Error> {
        let mut login = HashMap::new();
        login.insert("username", "Looouiiis");
        login.insert("password", "Efzxlyc020722");
        let request = self
            .client
            .post(self.base.join("auth/login").unwrap())
            .json(&login)
            .build()?;

        let mut res = self.client.execute(request.try_clone().unwrap()).await.ok();
        while res.is_none() {
            res = self.client.execute(request.try_clone().unwrap()).await.ok();
            sleep(Duration::from_secs(10)).await;
        }
        let res = res.unwrap();
        let json: serde_json::Value = res.json().await?;
        let mut header = HeaderMap::new();
        let mut token = String::from("Bearer ");
        token.push_str(json["token"].as_str().unwrap_or_default());
        header.insert(
            "x-nd-authorization",
            HeaderValue::from_str(token.as_str()).unwrap(),
        );
        header.insert(
            "x-nd-client-unique-id",
            HeaderValue::from_str(json["id"].as_str().unwrap_or_default()).unwrap(),
        );
        self.client = ClientBuilder::new().default_headers(header).build()?;
        self.state = ConnectionState::Connected;
        Ok(())
    }

    pub async fn get_songs(&self) -> Result<(), tauri_plugin_http::Error> {
        let res = self
            .client
            .get(self.base.join("api/album").unwrap())
            .query(&[("_start", "0"), ("_end", "0"), ("_sort", "title")])
            .send()
            .await?;
        let json: serde_json::Value = res.json().await.unwrap();
        info!("{:?}", json);
        Ok(())
    }
}
