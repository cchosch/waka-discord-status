use std::env;

use lazy_static::lazy_static;
use tower::layer::util::{Identity, Stack};
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;

use crate::db::{ConnPool, DbConn};
use crate::err::{ApiError, ApiResult};

pub const PROD: bool = cfg!(not(debug_assertions));

#[derive(Clone, Debug)]
pub struct AppVars {
    // pub database_url: String,
    pub waka_key: String,
    pub discord_key: String,
}

lazy_static! {
    pub static ref VARS: AppVars = new_vars();
}

fn new_vars() -> AppVars {
    AppVars {
        // database_url: env::var("DATABASE_URL").unwrap(),
        waka_key: env::var("WAKA_KEY").unwrap(),
        discord_key: env::var("DISCORD_KEY").unwrap(),
    }
}
#[derive(Clone)]
pub struct ApiContext {
    pub pool: ConnPool,
}

impl ApiContext {
    pub async fn get_conn(&self) -> ApiResult<DbConn> {
        self.pool.get().await.map_err(|x| {
            println!("{x}");
            ApiError::InternalError
        })
    }
}

pub fn api_context_layer(
    pool: ConnPool,
) -> ServiceBuilder<Stack<AddExtensionLayer<ApiContext>, Identity>> {
    ServiceBuilder::new().layer(AddExtensionLayer::new(ApiContext { pool }))
}
