use crate::{cron, labels, secrets};
use k8s_openapi::api::core::v1::Secret;
use kube::runtime::controller::Action;
use kube::runtime::watcher::Config;
use kube::runtime::Controller;
use kube::{Api, ResourceExt};
use std::sync::Arc;
use std::time::Duration;

use crate::k8s::K8s;
use futures::StreamExt;
use kube::api::ListParams;
use tracing::info;

#[derive(thiserror::Error, Debug)]
pub enum Error {}
pub type Result<T, E = Error> = std::result::Result<T, E>;

pub(crate) async fn reconcile(obj: Arc<Secret>, k8s: Arc<K8s>) -> Result<Action> {
    info!("reconcile request: {}", obj.name_any());
    if labels::managed_by_us(&obj) {
        secrets::update(&obj, &k8s).await;
        cron::update(&obj, &k8s).await
    }
    Ok(Action::requeue(Duration::from_secs(3600)))
}

pub(crate) fn error_policy(_object: Arc<Secret>, _err: &Error, _k8s: Arc<K8s>) -> Action {
    Action::requeue(Duration::from_secs(5))
}

pub async fn run_with_reconciliation(k8s: K8s) {
    let client = K8s::get_client().await;
    let secrets = Api::<Secret>::all(client);
    let watcher_config = Config {
        label_selector: Some(labels::get_managed_label()),
        ..Default::default()
    };
    Controller::new(secrets.clone(), watcher_config)
        .shutdown_on_signal()
        .run(reconcile, error_policy, Arc::new(k8s))
        .filter_map(|x| async move { std::result::Result::ok(x) })
        .for_each(|_| futures::future::ready(()))
        .await;
}

pub async fn run_one_shot(k8s: K8s) {
    let client = K8s::get_client().await;
    let secrets = Api::<Secret>::all(client);
    for secret in secrets.list(&ListParams::default()).await.unwrap() {
        let _ = reconcile(Arc::new(secret), Arc::new(k8s)).await;
    }
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

    use crate::cron::build_cron_name;
    use crate::k8s::K8s;
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

    fn build_managed_secret_with_annotations(
        name: String,
        annotations: Vec<(String, String)>,
    ) -> Secret {
        let mut label_map = BTreeMap::new();
        label_map.insert(
            "v1.secret.runo.rocks/managed".to_string(),
            "true".to_string(),
        );
        let annotation_map = annotations
            .into_iter()
            .collect::<BTreeMap<String, String>>();
        Secret {
            metadata: ObjectMeta {
                labels: Some(label_map),
                annotations: Some(annotation_map),
                name: Some(name),
                namespace: Some("default".to_string()),
                ..ObjectMeta::default()
            },
            ..Secret::default()
        }
    }

    fn build_unmanaged_secret_with_annotations(
        name: String,
        annotations: Vec<(String, String)>,
    ) -> Secret {
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
        let k8s = Arc::new((K8s::new(false)));

        let key_0 = String::from("v1.secret.runo.rocks/generate-0");
        let value_0 = String::from("username");

        let key_1 = String::from("v1.secret.runo.rocks/generate-1");
        let value_1 = String::from("password");

        let post_params = build_post_params();
        let secret = build_managed_secret_with_annotations(
            secret_name.to_string(),
            vec![(key_0, value_0), (key_1, value_1)],
        );
        let secrets: Api<Secret> = Api::namespaced(client.clone(), "default");
        secrets.create(&post_params, &secret).await.unwrap();

        // Data should be empty
        assert!(secrets.get(secret_name).await.unwrap().data.is_none());

        // reconcile it
        reconcile(Arc::new(secret), k8s).await.unwrap();

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
    async fn integration_reconcile_should_not_generate_secret_when_unmanaged() {
        let secret_name = "runo-generate-test-unmanaged";
        let config = Config::from_kubeconfig(&get_kubeconfig_options())
            .await
            .unwrap();
        let client = Client::try_from(config).unwrap();
        let k8s = Arc::new((K8s::new(false)));

        let key_0 = String::from("v1.secret.runo.rocks/generate-0");
        let value_0 = String::from("username");

        let key_1 = String::from("v1.secret.runo.rocks/generate-1");
        let value_1 = String::from("password");

        let post_params = build_post_params();
        let secret = build_unmanaged_secret_with_annotations(
            secret_name.to_string(),
            vec![(key_0, value_0), (key_1, value_1)],
        );
        let secrets: Api<Secret> = Api::namespaced(client.clone(), "default");
        secrets.create(&post_params, &secret).await.unwrap();

        // Data should be empty
        assert!(secrets.get(secret_name).await.unwrap().data.is_none());

        // reconcile it
        reconcile(Arc::new(secret), k8s).await.unwrap();

        // Data should still be empty
        assert!(secrets.get(secret_name).await.unwrap().data.is_none());
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
        let k8s = Arc::new((K8s::new(false)));

        let key_1 = String::from("v1.secret.runo.rocks/generate-0");
        let value_1 = String::from("username");

        let key_2 = String::from("v1.secret.runo.rocks/length-0");
        let value_2 = String::from("10");

        let post_params = build_post_params();
        let secret = build_managed_secret_with_annotations(
            secret_name.to_string(),
            vec![(key_1, value_1), (key_2, value_2)],
        );
        let secrets: Api<Secret> = Api::namespaced(client.clone(), "default");
        secrets.create(&post_params, &secret).await.unwrap();

        // Data should be empty
        assert!(secrets.get(secret_name).await.unwrap().data.is_none());

        // reconcile it
        reconcile(Arc::new(secret), k8s).await.unwrap();

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
        let k8s = Arc::new((K8s::new(false)));

        let key_1 = String::from("v1.secret.runo.rocks/generate-0");
        let value_1 = String::from("username");

        let key_2 = String::from("v1.secret.runo.rocks/charset-0");
        let value_2 = String::from("abcd");

        let post_params = build_post_params();
        let secret = build_managed_secret_with_annotations(
            secret_name.to_string(),
            vec![(key_1, value_1), (key_2, value_2)],
        );
        let secrets: Api<Secret> = Api::namespaced(client.clone(), "default");
        secrets.create(&post_params, &secret).await.unwrap();

        // Data should be empty
        assert!(secrets.get(secret_name).await.unwrap().data.is_none());

        // reconcile it
        reconcile(Arc::new(secret), k8s).await.unwrap();

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
        let k8s = Arc::new((K8s::new(false)));

        let key_1 = String::from("v1.secret.runo.rocks/generate-0");
        let value_1 = String::from("username");

        let key_2 = String::from("v1.secret.runo.rocks/pattern-0");
        let value_2 = String::from("\\S");

        let post_params = build_post_params();
        let secret = build_managed_secret_with_annotations(
            secret_name.to_string(),
            vec![(key_1, value_1), (key_2, value_2)],
        );
        let secrets: Api<Secret> = Api::namespaced(client.clone(), "default");
        secrets.create(&post_params, &secret).await.unwrap();

        // Data should be empty
        assert!(secrets.get(secret_name).await.unwrap().data.is_none());

        // reconcile it
        reconcile(Arc::new(secret), k8s).await.unwrap();

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
    async fn integration_reconcile_should_generate_secret_with_renewal() {
        let secret_name = "runo-generate-test-renewal";
        let config = Config::from_kubeconfig(&get_kubeconfig_options())
            .await
            .unwrap();
        let client = Client::try_from(config).unwrap();
        let k8s = Arc::new((K8s::new(false)));

        let key_1 = String::from("v1.secret.runo.rocks/generate-0");
        let value_1 = String::from("username");

        let key_2 = String::from("v1.secret.runo.rocks/renewal-cron-0");
        let value_2 = String::from("* * * * *");

        let post_params = build_post_params();
        let secret = build_managed_secret_with_annotations(
            secret_name.to_string(),
            vec![(key_1, value_1), (key_2, value_2)],
        );
        let secrets: Api<Secret> = Api::namespaced(client.clone(), "default");
        secrets.create(&post_params, &secret).await.unwrap();
        // Data should be empty
        assert!(secrets.get(secret_name).await.unwrap().data.is_none());

        // reconcile it
        reconcile(Arc::new(secret.clone()), k8s.clone())
            .await
            .unwrap();
        let secret_before_cron = secrets.get(secret_name).await.unwrap().data.unwrap();
        let username_before_cron =
            from_utf8(&secret_before_cron.get("username").unwrap().0).unwrap();
        sleep(Duration::from_secs(60)).await;

        // check if renewal annotation is set
        let secret_with_renewal_annotation = secrets
            .get(secret_name)
            .await
            .unwrap()
            .metadata
            .annotations
            .unwrap();
        assert_eq!(
            secret_with_renewal_annotation
                .get("v1.secret.runo.rocks/renewal-0")
                .unwrap(),
            "true"
        );

        // reconcile again to renewal secret
        reconcile(Arc::new(secret.clone()), k8s).await.unwrap();
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
                build_cron_name(&Arc::new(secret), "0").as_str(),
                &DeleteParams::default(),
            )
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn integration_reconcile_should_not_generate_secret_when_dry_run() {
        let secret_name = "runo-generate-test-generate-dry-run";
        let config = Config::from_kubeconfig(&get_kubeconfig_options())
            .await
            .unwrap();
        let client = Client::try_from(config).unwrap();
        let k8s = Arc::new((K8s::new(true)));

        let key_0 = String::from("v1.secret.runo.rocks/generate-0");
        let value_0 = String::from("username");

        let post_params = build_post_params();
        let secret =
            build_managed_secret_with_annotations(secret_name.to_string(), vec![(key_0, value_0)]);
        let secrets: Api<Secret> = Api::namespaced(client.clone(), "default");
        secrets.create(&post_params, &secret).await.unwrap();

        // Data should be empty
        assert!(secrets.get(secret_name).await.unwrap().data.is_none());

        // reconcile it
        reconcile(Arc::new(secret), k8s).await.unwrap();

        // Value for field username should not be generated
        assert!(secrets.get(secret_name).await.unwrap().data.is_none());

        secrets
            .delete(secret_name, &DeleteParams::default())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn integration_reconcile_should_not_generate_cronjob_when_dry_run() {
        let secret_name = "runo-generate-test-generate-cronjob-dry-run";
        let config = Config::from_kubeconfig(&get_kubeconfig_options())
            .await
            .unwrap();
        let client = Client::try_from(config).unwrap();
        let k8s = Arc::new((K8s::new(true)));

        let key_0 = String::from("v1.secret.runo.rocks/generate-0");
        let value_0 = String::from("username");

        let key_1 = String::from("v1.secret.runo.rocks/renewal-cron-0");
        let value_1 = String::from("* * * * *");

        let post_params = build_post_params();
        let secret = build_managed_secret_with_annotations(
            secret_name.to_string(),
            vec![(key_0, value_0), (key_1, value_1)],
        );
        let secrets: Api<Secret> = Api::namespaced(client.clone(), "default");
        let cronjobs: Api<CronJob> = Api::namespaced(client.clone(), "default");
        secrets.create(&post_params, &secret).await.unwrap();

        // Data should be empty
        assert!(secrets.get(secret_name).await.unwrap().data.is_none());

        // reconcile it
        reconcile(Arc::new(secret.clone()), k8s).await.unwrap();

        assert!(cronjobs
            .get(build_cron_name(&Arc::new(secret), "0").as_str())
            .await
            .is_err());

        secrets
            .delete(secret_name, &DeleteParams::default())
            .await
            .unwrap();
    }
}
