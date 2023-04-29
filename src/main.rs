mod annotations;
mod cron;
mod errors;
mod logging;
mod reconciler;
mod secrets;

use futures::StreamExt;
use k8s_openapi::api::core::v1::Secret;

use kube::{runtime::controller::Controller, Api, Client};
use std::sync::Arc;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), kube::Error> {
    match logging::set_logger() {
        true => info!("Logging initialized.."),
        false => panic!("Logging not initialized properly!. Exiting..."),
    }
    let client = Client::try_default().await?;
    let secrets = Api::<Secret>::all(client);

    Controller::new(secrets.clone(), Default::default())
        .run(
            reconciler::reconcile,
            reconciler::error_policy,
            Arc::new(()),
        )
        .for_each(|_| futures::future::ready(()))
        .await;
    Ok(())
}
