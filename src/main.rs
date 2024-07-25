use chrono::{DateTime, Utc};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde::{Deserialize, Serialize};
use std::future::IntoFuture;

pub use util::PROD;
pub use util::VARS;
use crate::main_loop::main_loop;

pub mod util;
mod main_loop;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DurationData {
    time: f64,
    project: String,
    duration: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DurationResponse {
    data: Vec<DurationData>,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    timezone: String,
}


#[tokio::main]
async fn main() {
    if !PROD {
        dotenvy::dotenv().unwrap();
    }

    let vars = VARS.clone();
    let waka_client = reqwest::ClientBuilder::new()
        .default_headers(HeaderMap::from_iter(vec![(
            AUTHORIZATION,
            HeaderValue::from_str(format!("Basic {}", vars.waka_key).as_str()).unwrap(),
        )]))
        .build()
        .unwrap();

    let discord_client = reqwest::ClientBuilder::new()
        .default_headers(HeaderMap::from_iter(vec![(
            AUTHORIZATION,
            HeaderValue::from_str(vars.discord_key.clone().as_str()).unwrap(),
        )]))
        .build()
        .unwrap();
    if let Err(e) = main_loop(waka_client, discord_client).await {
        eprintln!("main loop returned {e}")
    }
}
