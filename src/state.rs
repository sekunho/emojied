use crate::db;
use crate::config;

#[derive(Clone)]
pub struct AppState {
    pub db_handle: db::Handle,
    pub config: config::AppConfig,
}
