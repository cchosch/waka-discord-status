use std::time::Duration;
use chrono::Local;
use serde_json::json;
use crate::DurationResponse;

fn format_time(total_duration: f64) -> String {
    let hours = total_duration / 60f64 / 60f64;
    let minutes = (total_duration / 60f64) % (60f64);
    if hours == 0f64 {
        return format!("{}m", minutes as i32);
    }

    format!("{}h {}m", hours as i32, minutes as i32)
}

pub const BASE_WAKA_API_URL: &'static str = "https://api.wakatime.com/api/v1";
pub const BASE_DISCORD_API_URL: &'static str = "https://discord.com/api/v10";
pub async fn main_loop(waka_client: reqwest::Client, discord_client: reqwest::Client) -> anyhow::Result<()> {
    let mut interval = tokio::time::interval(Duration::from_secs(15));
    let mut last_duration: Option<String> = None;
    loop {
        interval.tick().await;

        // get recent durations
        let resp = waka_client
            .get(format!(
                "{}/users/current/durations?date={}",
                BASE_WAKA_API_URL,
                Local::now().format("%Y-%m-%d")
            ))
            .send()
            .await?
            .json::<DurationResponse>()
            .await?;

        // get duration string for comparison
        let duration_str = format_time(
            resp.data
                .iter()
                .fold(0f64, |prev, curr| prev + curr.duration),
        );

        // if last duration is the same as current duration string, continue
        if last_duration.as_ref().is_some_and(|last| *last == duration_str) {
            continue;
        }
        last_duration = Some(duration_str.clone());

        match discord_client
            .patch(format!("{}/users/@me/settings", BASE_DISCORD_API_URL))
            .json(&json!({
                "custom_status": {
                    "text": format!("coding for {}", duration_str),
                }
            }))
            .send()
            .await
        {
            Err(e) => {
                eprintln!("Discord client issue {e}");
            }
            Ok(_) => {}
        };
    }
}
