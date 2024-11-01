use actix_web::dev::ServiceResponse;
use actix_web::{middleware, web, App, HttpRequest, HttpServer, Responder};
use actix_web::middleware::{ErrorHandlerResponse, Logger};
use env_logger::Env;
use sys::app_set;

use crate::sys::app_set::AppSet;
use crate::sys::init::AppConfig;

mod sys;

#[actix_web::get("/")]
async fn index(app_set: web::Data<AppSet>, req: HttpRequest) -> impl Responder {
    "Hello, world!"
}


fn err_handler<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>, actix_web::Error> {
    let app_set = res.request().app_data::<web::Data<AppSet>>().unwrap();
    let response = render_error_page(res);
    Ok(ErrorHandlerResponse::Response(res.into_response(response.map_into_left_body())))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let app_config = AppConfig::new();

    let app_set_instance = AppSet::new(app_config.clone()).await;

    let app_set = web::Data::new(app_set_instance);
    
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(middleware::ErrorHandlers::new().default_handler(err_handler))
            .app_data(app_set.clone())
            .service(index)
    })
    .bind(app_config.server_bind.clone())?
    .workers(app_config.server_workers.clone())
    .backlog(app_config.server_backlog.clone())
    .run();
    
    server.await?;

    Ok(())
}