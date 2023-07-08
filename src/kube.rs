use kube::api::PatchParams;
use kube::Client;

pub fn get_patch_params() -> PatchParams {
    PatchParams {
        dry_run: false,
        force: true,
        field_manager: Some("runo".to_string()),
        field_validation: None,
    }
}

pub async fn get_client() -> Client {
    match Client::try_default().await {
        Ok(client) => client,
        Err(e) => panic!("Can't create Kubernetes client. Exiting...\n {:?}", e),
    }
}
