use std::env;

use lazy_static::lazy_static;
use tower::layer::util::{Identity, Stack};
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;

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
        discord_key: env::var("DISCORD_KEY").unwrap(),
    }
}
