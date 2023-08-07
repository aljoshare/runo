mod annotations;
mod cron;
mod errors;
mod http;
mod k8s;
mod logging;
mod reconciler;
mod secrets;

use crate::k8s::K8s;
use anyhow::anyhow;
use clap::Parser;
use tracing::info;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct MainArgs {
    #[clap(long, default_value_t = 8080)]
    http_port: u16,
    #[clap(long, default_value_t = false)]
    dry_run: bool,
    #[clap(long, default_value_t = String::from("reconciliation"))]
    mode: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = MainArgs::parse();
    match logging::set_logger() {
        true => info!("Logging initialized.."),
        false => panic!("Logging not initialized properly!. Exiting..."),
    }
    let k8s = K8s::new(args.dry_run);
    match args.mode.as_str() {
        "reconciliation" => {
            info!("Running runo in reconciliation mode.");
            let http_server_result = http::run_http_server(args.http_port);
            let reconciler = reconciler::run_with_reconciliation(k8s);
            match http_server_result {
                Ok(http_server) => {
                    tokio::join!(reconciler, http_server).1.unwrap();
                    Ok(())
                }
                Err(_) => Err(anyhow!("Can't bind HTTP server to port!")),
            }
        }
        "one-shot" => {
            info!("Running runo in one-shot mode.");
            reconciler::run_one_shot(k8s).await;
            Ok(())
        }
        _ => Err(anyhow!("Mode is not supported!: {:?}", args.mode)),
    }
}
