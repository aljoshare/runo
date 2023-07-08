use crate::{annotations, cron, secrets};
use k8s_openapi::api::core::v1::Secret;
use kube::runtime::controller::Action;
use kube::runtime::Controller;
use kube::{Api, ResourceExt};
use std::sync::Arc;
use std::time::Duration;

use crate::kube::get_client;
use futures::StreamExt;
use tracing::info;

#[derive(thiserror::Error, Debug)]
pub enum Error {}
pub type Result<T, E = Error> = std::result::Result<T, E>;

pub(crate) async fn reconcile(obj: Arc<Secret>, _ctx: Arc<()>) -> Result<Action> {
    info!("reconcile request: {}", obj.name_any());
    if annotations::has_our_annotations(&obj) {
        secrets::update(&obj).await;
        cron::update(&obj).await
    }
    Ok(Action::requeue(Duration::from_secs(3600)))
}

pub(crate) fn error_policy(_object: Arc<Secret>, _err: &Error, _ctx: Arc<()>) -> Action {
    Action::requeue(Duration::from_secs(5))
}

pub async fn run() {
    let client = get_client().await;
    let secrets = Api::<Secret>::all(client);
    Controller::new(secrets.clone(), Default::default())
        .shutdown_on_signal()
        .run(reconcile, error_policy, Arc::new(()))
        .filter_map(|x| async move { std::result::Result::ok(x) })
        .for_each(|_| futures::future::ready(()))
        .await;
}

#[cfg(test)]
mod tests {
    use crate::reconciler::reconcile;
    use k8s_openapi::api::core::v1::Secret;
    use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    use kube::api::{DeleteParams, PostParams};
    use kube::config::KubeConfigOptions;

    use kube::{Api, Client, Config};
    use regex::Regex;
    use std::collections::BTreeMap;

    use k8s_openapi::api::batch::v1::CronJob;
    use std::str::from_utf8;
    use std::sync::Arc;
    use std::time::Duration;

    use crate::cron::generate_cron_name;
    use tokio::time::sleep;

    fn get_kubeconfig_options() -> KubeConfigOptions {
        KubeConfigOptions {
            context: Some("kind-kind".to_string()),
            cluster: Some("kind-kind".to_string()),
            ..KubeConfigOptions::default()
        }
    }

    fn build_post_params() -> PostParams {
        PostParams {
            dry_run: false,
            field_manager: Some("runo-integration-tests".to_string()),
        }
    }

    fn build_secret_with_annotations(name: String, annotations: Vec<(String, String)>) -> Secret {
        let annotation_map = annotations
            .into_iter()
            .collect::<BTreeMap<String, String>>();
        Secret {
            metadata: ObjectMeta {
                annotations: Some(annotation_map),
                name: Some(name),
                namespace: Some("default".to_string()),
                ..ObjectMeta::default()
            },
            ..Secret::default()
        }
    }

    #[tokio::test]
    async fn integration_reconcile_should_generate_secret() {
        let secret_name = "runo-generate-test-generate";
        let config = Config::from_kubeconfig(&get_kubeconfig_options())
            .await
            .unwrap();
        let client = Client::try_from(config).unwrap();
        let ctx = Arc::new(());

        let key_0 = String::from("v1.secret.runo.rocks/generate-0");
        let value_0 = String::from("username");

        let key_1 = String::from("v1.secret.runo.rocks/generate-1");
        let value_1 = String::from("password");

        let post_params = build_post_params();
        let secret = build_secret_with_annotations(
            secret_name.to_string(),
            vec![(key_0, value_0), (key_1, value_1)],
        );
        let secrets: Api<Secret> = Api::namespaced(client.clone(), "default");
        secrets.create(&post_params, &secret).await.unwrap();

        // Data should be empty
        assert!(secrets.get(secret_name).await.unwrap().data.is_none());

        // reconcile it
        reconcile(Arc::new(secret), ctx).await.unwrap();

        // Value for field username should be generated
        assert!(secrets
            .get(secret_name)
            .await
            .unwrap()
            .data
            .unwrap()
            .get("username")
            .is_some());
        // Value for field password should be generated
        assert!(secrets
            .get(secret_name)
            .await
            .unwrap()
            .data
            .unwrap()
            .get("password")
            .is_some());
        secrets
            .delete(secret_name, &DeleteParams::default())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn integration_reconcile_should_generate_secret_with_length() {
        let secret_name = "runo-generate-test-length";
        let config = Config::from_kubeconfig(&get_kubeconfig_options())
            .await
            .unwrap();
        let client = Client::try_from(config).unwrap();
        let ctx = Arc::new(());

        let key_1 = String::from("v1.secret.runo.rocks/generate-0");
        let value_1 = String::from("username");

        let key_2 = String::from("v1.secret.runo.rocks/length-0");
        let value_2 = String::from("10");

        let post_params = build_post_params();
        let secret = build_secret_with_annotations(
            secret_name.to_string(),
            vec![(key_1, value_1), (key_2, value_2)],
        );
        let secrets: Api<Secret> = Api::namespaced(client.clone(), "default");
        secrets.create(&post_params, &secret).await.unwrap();

        // Data should be empty
        assert!(secrets.get(secret_name).await.unwrap().data.is_none());

        // reconcile it
        reconcile(Arc::new(secret), ctx).await.unwrap();

        // Value for field username should be generated and has length of 10
        let secret = secrets.get(secret_name).await.unwrap().data.unwrap();
        let username = from_utf8(&secret.get("username").unwrap().0).unwrap();
        assert_eq!(username.chars().count(), 10);
        secrets
            .delete(secret_name, &DeleteParams::default())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn integration_reconcile_should_generate_secret_with_charset() {
        let secret_name = "runo-generate-test-charset";
        let config = Config::from_kubeconfig(&get_kubeconfig_options())
            .await
            .unwrap();
        let client = Client::try_from(config).unwrap();
        let ctx = Arc::new(());

        let key_1 = String::from("v1.secret.runo.rocks/generate-0");
        let value_1 = String::from("username");

        let key_2 = String::from("v1.secret.runo.rocks/charset-0");
        let value_2 = String::from("abcd");

        let post_params = build_post_params();
        let secret = build_secret_with_annotations(
            secret_name.to_string(),
            vec![(key_1, value_1), (key_2, value_2)],
        );
        let secrets: Api<Secret> = Api::namespaced(client.clone(), "default");
        secrets.create(&post_params, &secret).await.unwrap();

        // Data should be empty
        assert!(secrets.get(secret_name).await.unwrap().data.is_none());

        // reconcile it
        reconcile(Arc::new(secret), ctx).await.unwrap();

        // Value for field username should be generated and match the charset
        let secret = secrets.get(secret_name).await.unwrap().data.unwrap();
        let username = from_utf8(&secret.get("username").unwrap().0).unwrap();
        let re = Regex::new(r"[abcd]+").unwrap();
        assert!(re.is_match(username));

        secrets
            .delete(secret_name, &DeleteParams::default())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn integration_reconcile_should_generate_secret_with_pattern() {
        let secret_name = "runo-generate-test-pattern";
        let config = Config::from_kubeconfig(&get_kubeconfig_options())
            .await
            .unwrap();
        let client = Client::try_from(config).unwrap();
        let ctx = Arc::new(());

        let key_1 = String::from("v1.secret.runo.rocks/generate-0");
        let value_1 = String::from("username");

        let key_2 = String::from("v1.secret.runo.rocks/pattern-0");
        let value_2 = String::from("\\S");

        let post_params = build_post_params();
        let secret = build_secret_with_annotations(
            secret_name.to_string(),
            vec![(key_1, value_1), (key_2, value_2)],
        );
        let secrets: Api<Secret> = Api::namespaced(client.clone(), "default");
        secrets.create(&post_params, &secret).await.unwrap();

        // Data should be empty
        assert!(secrets.get(secret_name).await.unwrap().data.is_none());

        // reconcile it
        reconcile(Arc::new(secret), ctx).await.unwrap();

        // Value for field username should be generated and match the pattern
        let secret = secrets.get(secret_name).await.unwrap().data.unwrap();
        let username = from_utf8(&secret.get("username").unwrap().0).unwrap();
        let re = Regex::new(r"[\S]+").unwrap();
        assert!(re.is_match(username));

        secrets
            .delete(secret_name, &DeleteParams::default())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn integration_reconcile_should_generate_secret_with_regeneration() {
        let secret_name = "runo-generate-test-regeneration";
        let config = Config::from_kubeconfig(&get_kubeconfig_options())
            .await
            .unwrap();
        let client = Client::try_from(config).unwrap();
        let ctx = Arc::new(());

        let key_1 = String::from("v1.secret.runo.rocks/generate-0");
        let value_1 = String::from("username");

        let key_2 = String::from("v1.secret.runo.rocks/regeneration-cron-0");
        let value_2 = String::from("* * * * *");

        let post_params = build_post_params();
        let secret = build_secret_with_annotations(
            secret_name.to_string(),
            vec![(key_1, value_1), (key_2, value_2)],
        );
        let secrets: Api<Secret> = Api::namespaced(client.clone(), "default");
        secrets.create(&post_params, &secret).await.unwrap();
        // Data should be empty
        assert!(secrets.get(secret_name).await.unwrap().data.is_none());

        // reconcile it
        reconcile(Arc::new(secret.clone()), ctx.clone())
            .await
            .unwrap();
        let secret_before_cron = secrets.get(secret_name).await.unwrap().data.unwrap();
        let username_before_cron =
            from_utf8(&secret_before_cron.get("username").unwrap().0).unwrap();
        sleep(Duration::from_secs(60)).await;

        // check if regeneration annotation is set
        let secret_with_regeneration_annotation = secrets
            .get(secret_name)
            .await
            .unwrap()
            .metadata
            .annotations
            .unwrap();
        assert_eq!(
            secret_with_regeneration_annotation
                .get("v1.secret.runo.rocks/regenerate-0")
                .unwrap(),
            "true"
        );

        // reconcile again to regenerate secret
        reconcile(Arc::new(secret.clone()), ctx).await.unwrap();
        let secret_after_cron = secrets.get(secret_name).await.unwrap().data.unwrap();
        let username_after_cron = from_utf8(&secret_after_cron.get("username").unwrap().0).unwrap();
        assert_ne!(username_before_cron, username_after_cron);
        // Cleanup secrets and generated cronjobs
        secrets
            .delete(secret_name, &DeleteParams::default())
            .await
            .unwrap();
        let cronjobs: Api<CronJob> = Api::namespaced(client.clone(), "default");
        cronjobs
            .delete(
                generate_cron_name(&Arc::new(secret), "0").as_str(),
                &DeleteParams::default(),
            )
            .await
            .unwrap();
    }
}
