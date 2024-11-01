#[derive(Clone)]
pub struct AppConfig {
    pub server_bind: String,
    pub server_backlog: u32,
    pub server_workers: usize,
}

impl AppConfig {
    pub fn new() -> Self {
        let app_config = AppConfig {
            server_bind: "0.0.0.0:83".to_string(),
            server_backlog: 512,
            server_workers: 16,
        };
        app_config
    }
}