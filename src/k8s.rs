use kube::api::{PatchParams, PostParams};
use kube::Client;
use tracing::info;

#[derive(Copy, Clone)]
pub struct K8s {
    pub(crate) dry_run: bool,
}

impl K8s {
    pub fn new(dry_run: bool) -> K8s {
        if dry_run {
            info!("Running runo in dry-run mode!")
        }
        K8s { dry_run }
    }
    pub fn get_patch_params(self) -> PatchParams {
        PatchParams {
            dry_run: self.dry_run,
            force: true,
            field_manager: Some("runo".to_string()),
            field_validation: None,
        }
    }

    pub fn get_post_params(self) -> PostParams {
        PostParams {
            dry_run: self.dry_run,
            field_manager: Some("runo".to_string()),
        }
    }

    pub async fn get_client() -> Client {
        match Client::try_default().await {
            Ok(client) => client,
            Err(e) => panic!("Can't create Kubernetes client. Exiting...\n {:?}", e),
        }
    }
}
