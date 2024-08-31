mod annotations;
mod config;
mod cron;
mod errors;
mod http;
mod k8s;
mod labels;
mod logging;
mod reconciler;
mod secrets;

use crate::k8s::K8s;
use anyhow::anyhow;
use clap::Parser;
use config::RunoConfig;
use errors::LogLevelMissing;
use tracing::info;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct MainArgs {
    #[clap(long, default_value_t = 8080)]
    http_port: u16,
    #[clap(long, default_value_t = false)]
    dry_run: bool,
    #[clap(long, default_value_t = String::from("reconciliation"))]
    mode: String,
    #[clap(long, default_value_t = 300)]
    requeue_duration: u64,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = MainArgs::parse();
    let subscriber = match logging::get_subscriber(true) {
        Ok(s) => {
            info!("Logging initialized..");
            s
        }
        Err(LogLevelMissing) => panic!("RUST_LOG is not set properly!"),
    };
    subscriber.init();
    let k8s = K8s::build(args.dry_run);
    let config = RunoConfig::build(k8s, args.requeue_duration);
    match args.mode.as_str() {
        "reconciliation" => {
            info!("Running runo in reconciliation mode.");
            let http_server_result = http::run_http_server(args.http_port);
            let reconciler = reconciler::run_with_reconciliation(config);
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
            reconciler::run_one_shot(config).await;
            Ok(())
        }
        _ => Err(anyhow!("Mode is not supported!: {:?}", args.mode)),
    }
}
