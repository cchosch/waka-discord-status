use chrono::{DateTime, Local, TimeDelta, Utc};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde::{Deserialize, Serialize};
use std::future::IntoFuture;
use std::time::Duration;

pub use util::PROD;
pub use util::VARS;
pub const BASE_WAKA_API_URL: &'static str = "https://api.wakatime.com/api/v1";

mod db;
pub mod err;
pub mod util;

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

fn format_time(total_duration: f64) -> String {
    let hours = total_duration / 60f64 / 60f64;
    let minutes = (total_duration / 60f64) % (60f64);
    let seconds = total_duration % (60f64);

    format!("{}h {}m {}s", hours as i32, minutes as i32, seconds as i32)
}

#[tokio::main]
async fn main() {
    if !PROD {
        dotenvy::dotenv().unwrap();
    }

    let vars = VARS.clone();
    let mut interval = tokio::time::interval(Duration::from_secs(2));
    let mut client = reqwest::ClientBuilder::new()
        .default_headers(HeaderMap::from_iter(vec![(
            AUTHORIZATION,
            HeaderValue::from_str(format!("Basic {}", vars.waka_key).as_str()).unwrap(),
        )]))
        .build()
        .unwrap();
    let mut last_duration: Option<String> = None;
    loop {
        interval.tick().await;
        let resp = client
            .get(format!(
                "{}/users/current/durations?date={}",
                BASE_WAKA_API_URL,
                Utc::now().format("%Y-%m-%d")
            ))
            .send()
            .await
            .unwrap();
        let resp = resp.json::<DurationResponse>().await.unwrap();
        let duration_str = format_time(
            resp.data
                .iter()
                .fold(0f64, |prev, curr| prev + curr.duration),
        );

        match last_duration.as_ref() {
            Some(last_dur) => {
                if *last_dur == duration_str {
                    continue;
                }
            }
            _ => {}
        }
        last_duration = Some(duration_str.clone());
        println!("{}", duration_str);
    }
}
