use super::init::AppConfig;

pub struct AppSet {
    pub app_config: AppConfig,
}

impl AppSet {
    pub async  fn new(app_config: AppConfig) -> Self {
        let app_set = AppSet {
            app_config: app_config,
        };
        app_set
    }
}
