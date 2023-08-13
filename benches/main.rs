use criterion::{criterion_group, criterion_main, Criterion};
use std::collections::BTreeMap;

use assert_cmd::Command;
use k8s_openapi::api::core::v1::Secret;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::api::{DeleteParams, PostParams};
use kube::config::KubeConfigOptions;
use kube::{Api, Client, Config};

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

async fn create_secret(secret_name: String) {
    let config = Config::from_kubeconfig(&get_kubeconfig_options())
        .await
        .unwrap();
    let client = Client::try_from(config).unwrap();

    let key_1 = String::from("v1.secret.runo.rocks/generate-0");
    let value_1 = String::from("username");

    let key_2 = String::from("v1.secret.runo.rocks/length-0");
    let value_2 = String::from("10");

    let post_params = build_post_params();
    let secret = build_managed_secret_with_annotations(
        secret_name.clone(),
        vec![(key_1, value_1), (key_2, value_2)],
    );
    let secrets: Api<Secret> = Api::namespaced(client.clone(), "default");
    let _ = secrets.create(&post_params, &secret).await;
    assert!(secrets.get(secret_name.as_str()).await.is_ok());
}

async fn clear_secret(secret_name: String) {
    let config = Config::from_kubeconfig(&get_kubeconfig_options())
        .await
        .unwrap();
    let client = Client::try_from(config).unwrap();

    let key_1 = String::from("v1.secret.runo.rocks/generate-0");
    let value_1 = String::from("username");

    let key_2 = String::from("v1.secret.runo.rocks/length-0");
    let value_2 = String::from("10");

    let post_params = build_post_params();
    let secret = build_managed_secret_with_annotations(
        secret_name.clone(),
        vec![(key_1, value_1), (key_2, value_2)],
    );
    let secrets: Api<Secret> = Api::namespaced(client.clone(), "default");
    let _ = secrets
        .replace(secret_name.as_str(), &post_params, &secret)
        .await;
    assert!(secrets.get(secret_name.as_str()).await.is_ok());
    assert!(secrets
        .get(secret_name.as_str())
        .await
        .unwrap()
        .data
        .is_none());
}

async fn delete_secret(secret_name: String) {
    let config = Config::from_kubeconfig(&get_kubeconfig_options())
        .await
        .unwrap();
    let client = Client::try_from(config).unwrap();
    let secrets: Api<Secret> = Api::namespaced(client.clone(), "default");
    let _ = secrets
        .delete(secret_name.as_str(), &DeleteParams::default())
        .await;
    assert!(secrets.get(secret_name.as_str()).await.is_err());
}

async fn secret_generated(secret_name: String) {
    let config = Config::from_kubeconfig(&get_kubeconfig_options())
        .await
        .unwrap();
    let client = Client::try_from(config).unwrap();
    let secrets: Api<Secret> = Api::namespaced(client.clone(), "default");
    assert!(secrets
        .get(secret_name.as_str())
        .await
        .unwrap()
        .data
        .is_some());
}

async fn generated() {
    for i in 1..=20 {
        secret_generated(format!("runo-generate-performance-{:?}", i)).await
    }
}

async fn clear() {
    for i in 1..=20 {
        clear_secret(format!("runo-generate-performance-{:?}", i)).await
    }
}

async fn setup() {
    for i in 1..=20 {
        create_secret(format!("runo-generate-performance-{:?}", i)).await
    }
}

async fn teardown() {
    for i in 1..=20 {
        delete_secret(format!("runo-generate-performance-{:?}", i)).await
    }
}

fn runs_one_shot() {
    let runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
    let mut cmd = Command::cargo_bin("runo").unwrap();
    cmd.arg("--mode").arg("one-shot").assert().success();
    runtime.block_on(generated());
}

fn criterion_benchmark(c: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
    runtime.block_on(setup());
    c.bench_function("one-shot", |b| {
        b.iter(|| runs_one_shot());
        runtime.block_on(clear());
    });
    runtime.block_on(teardown());
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
