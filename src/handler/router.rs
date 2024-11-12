use std::collections::HashMap;

use actix_web::{HttpRequest, HttpResponse};
use bytes::Bytes;
use tera::Tera;

use crate::sys::init::AppConfig;

pub struct Router {
    pub template: Tera,
    pub static_cache: HashMap<String, Bytes>,
}

impl Router {
    pub fn new(app_config: &AppConfig, template: Tera, static_cach: HashMap<String, Bytes>) -> Self {
        let router = Router {
            template,
            static_cache: static_cach,
        };
        router
    }

    pub async fn handle_request(&self, req: HttpRequest) -> HttpResponse {
        let mut path = req.path().trim_start_matches('/').to_string();

        if path.ends_with('/') {
            path.push_str("index.html");
        }

        println!("Request path: {}", path);

        if let Some(content) = self.static_cache.get(&path) {
            self.handle_static_file(&path, content)
        } else {
            HttpResponse::NotFound().body("404 Not Found")
        }
    }

    fn handle_static_file(&self, path: &str, content: &Bytes) -> HttpResponse {
        if path.ends_with(".html") {
            self.render_template(path)
        } else {
            let mime_type = mime_guess::from_path(path).first_or_octet_stream();
            HttpResponse::Ok()
                .content_type(mime_type.as_ref())
                .body(content.clone())
        }
    }

    fn render_template(&self, path: &str) -> HttpResponse {
        let rendered = self.template.render(path, &tera::Context::new());
        match rendered {
            Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
            Err(_) => HttpResponse::InternalServerError().body("Template rendering error"),
        }
    }
}