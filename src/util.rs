use std::env;

use lazy_static::lazy_static;

pub const PROD: bool = cfg!(not(debug_assertions));

#[derive(Clone, Debug)]
pub struct AppVars {
    pub waka_key: String,
    pub discord_key: String,
}

lazy_static! {
    pub static ref VARS: AppVars = new_vars();
}

fn new_vars() -> AppVars {
    AppVars {
        waka_key: env::var("WAKA_KEY").unwrap(),
        discord_key: env::var("DISCORD_TOKEN").unwrap(),
    }
}
