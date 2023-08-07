use actix_web::dev::Server;
use actix_web::{get, middleware, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::io::Error;
use tracing::log::error;

#[get("/health")]
async fn health(_: HttpRequest) -> impl Responder {
    HttpResponse::Ok().json("healthy")
}

pub fn run_http_server(http_port: u16) -> Result<Server, Error> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default().exclude("/health"))
            .service(health)
    })
    .bind(format!("0.0.0.0:{:?}", http_port));
    match server {
        Ok(s) => Ok(s.shutdown_timeout(5).run()),
        Err(e) => {
            error!("Can't bind http server: {:?}", e);
            Err(e)
        }
    }
}
