mod annotations;
mod cron;
mod errors;
mod k8s;
mod logging;
mod reconciler;
mod secrets;

use crate::k8s::K8s;
use actix_web::{get, middleware, App, HttpRequest, HttpResponse, HttpServer, Responder};
use clap::Parser;
use tracing::info;

#[get("/health")]
async fn health(_: HttpRequest) -> impl Responder {
    HttpResponse::Ok().json("healthy")
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct MainArgs {
    #[clap(long, default_value_t = 8080)]
    http_port: u16,
    #[clap(long, default_value_t = false)]
    dry_run: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = MainArgs::parse();
    match logging::set_logger() {
        true => info!("Logging initialized.."),
        false => panic!("Logging not initialized properly!. Exiting..."),
    }
    let k8s = K8s::new(args.dry_run);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default().exclude("/health"))
            .service(health)
    })
    .bind(format!("0.0.0.0:{:?}", args.http_port))?
    .shutdown_timeout(5);

    let reconciler = reconciler::run(k8s);
    tokio::join!(reconciler, server.run()).1.unwrap();
    Ok(())
}
