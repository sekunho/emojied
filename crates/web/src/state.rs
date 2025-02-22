use crate::config;

pub struct AppEnv {
    pub db_handle: su_sqlite::handle::Handle,
    pub config: config::AppConfig,
}
