use std::{collections::HashMap, fs, path::Path};

use super::init::AppConfig;
use crate::handler::{err_page::ErrHandler, router::Router};
use bytes::Bytes;
use tera::Tera;

pub struct AppSet {
    pub app_config: AppConfig,
    pub err_handler: ErrHandler,
    pub handler: Router,
    pub template: Tera,
    pub static_cache: HashMap<String, Bytes>,
}

impl AppSet {
    pub async  fn new(app_config: AppConfig) -> Self {
        let static_cache = AppSet::load_cache_static_files(Path::new(&app_config.data_path));
        let template = AppSet::load_template_html(&static_cache);

        let app_set = AppSet {
            app_config: app_config.clone(),
            err_handler: ErrHandler::new(template.clone()).await,
            handler: Router::new(&app_config, template.clone(), static_cache.clone()),
            template,
            static_cache,
        };
        app_set
    }

    fn load_cache_static_files(dir: &Path) -> HashMap<String, Bytes> {
        let mut cache = HashMap::new();
        
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                            if let Ok(content) = fs::read(&path) {
                                cache.insert(filename.to_string(), Bytes::from(content));
                            }
                        }
                    }
                }
            }
        }

        cache
    }

    fn load_template_html(static_cach: &HashMap<String, Bytes>) -> Tera {
        let mut tera = Tera::default();
        for (filename, content) in static_cach {
            if filename.ends_with(".html") {
                if let Ok(template_content) = std::str::from_utf8(&content) {
                    tera.add_raw_template(filename, template_content).expect("Failed to add template");
                }
            }
        }
        tera
    }
}
