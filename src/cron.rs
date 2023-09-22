use crate::annotations;
use crate::annotations::{id_iter, renewal_cron, AnnotationResult};
use crate::errors::NoNamespaceForSecret;
use crate::k8s::K8s;
use k8s_openapi::api::batch::v1::{CronJob, CronJobSpec, JobSpec, JobTemplateSpec};
use k8s_openapi::api::core::v1::{
    Capabilities, Container, PodSpec, PodTemplateSpec, Secret, SecurityContext,
};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::{Api, ResourceExt};
use std::sync::Arc;
use tracing::{debug, error};

fn build_cronjob(obj: &Arc<Secret>, secret_name: &str, id: &str) -> CronJob {
    let cron_spec = renewal_cron(obj, id);
    debug!(
        "Will create cron job with pattern {:?} for {:?} and id {}",
        cron_spec.get_value(),
        obj.name_any(),
        id
    );

    CronJob {
        metadata: build_cronjob_object_meta(&build_cron_name(obj, id)),
        spec: build_cronjob_spec(cron_spec, secret_name, id),
        ..CronJob::default()
    }
}

fn build_cronjob_object_meta(cron_name: &str) -> ObjectMeta {
    ObjectMeta {
        name: Some(cron_name.into()),
        ..ObjectMeta::default()
    }
}

fn build_cronjob_spec(
    cron_spec: AnnotationResult<&str>,
    secret_name: &str,
    id: &str,
) -> Option<CronJobSpec> {
    Some(CronJobSpec {
        schedule: cron_spec.get_value().into(),
        job_template: JobTemplateSpec {
            spec: Some(JobSpec {
                template: PodTemplateSpec {
                    spec: build_pod_spec(secret_name, id),
                    ..PodTemplateSpec::default()
                },
                ..JobSpec::default()
            }),
            ..JobTemplateSpec::default()
        },
        ..CronJobSpec::default()
    })
}

fn build_pod_spec(secret_name: &str, id: &str) -> Option<PodSpec> {
    Some(PodSpec {
        containers: build_containers(secret_name, id),
        restart_policy: Some("Never".to_string()),
        service_account_name: Some("runo-cronjob".to_string()),
        ..PodSpec::default()
    })
}

fn build_containers(secret_name: &str, id: &str) -> Vec<Container> {
    vec![Container {
        args: Some(vec![
            "annotate".to_string(),
            "--overwrite".to_string(),
            "secrets".to_string(),
            secret_name.to_string(),
            format!("v1.secret.runo.rocks/renewal-{}=true", id),
        ]),
        command: None,
        image: Some("cgr.dev/chainguard/kubectl".to_string()),
        image_pull_policy: Some("Always".to_string()),
        name: "runo-cron-container".to_string(),
        resources: None,
        security_context: build_security_context(),
        ..Container::default()
    }]
}

fn build_security_context() -> Option<SecurityContext> {
    Some(SecurityContext {
        allow_privilege_escalation: Some(false),
        capabilities: Some(Capabilities {
            add: None,
            drop: Option::from(vec!["ALL".to_string()]),
        }),
        privileged: Some(false),
        read_only_root_filesystem: Some(true),
        run_as_group: Some(65534),
        run_as_non_root: Some(true),
        run_as_user: Some(65534),
        ..SecurityContext::default()
    })
}

async fn create_or_replace(cj: CronJob, namespace: &str, k8s: &K8s) {
    let cronjobs: Api<CronJob> = Api::namespaced(K8s::get_client().await, namespace);
    let c = cronjobs.create(&k8s.get_post_params(), &cj).await;
    match c {
        Err(_e) => {
            let r = cronjobs
                .replace(&cj.name_any(), &k8s.get_post_params(), &cj)
                .await;
            match r {
                Err(e) => error!("{:?}", e),
                Ok(cj) => debug!("{:?} replaced successfully", cj.metadata.name.unwrap()),
            }
        }
        Ok(cj) => debug!("{:?} created successfully", cj.metadata.name.unwrap()),
    }
}

pub fn build_cron_name(obj: &Arc<Secret>, id: &str) -> String {
    let mut trunc_obj_name = obj.name_any();
    trunc_obj_name.truncate(20);
    format!("runo-renewal-{}-{}", trunc_obj_name, id)
}

pub async fn update(obj: &Arc<Secret>, k8s: &K8s) {
    match obj.namespace() {
        Some(namespace) => {
            for id in id_iter(obj) {
                if annotations::has_cron(obj, &id) {
                    debug!(
                        "CronJob for {:?} and id {:?} needs to be created",
                        obj.name_any(),
                        id
                    );
                    let cj = build_cronjob(obj, obj.name_any().as_str(), &id);
                    create_or_replace(cj, &namespace, k8s).await
                }
            }
        }
        None => error!("{:?}", NoNamespaceForSecret),
    }
}

#[cfg(test)]
mod tests {
    use crate::cron::{build_cron_name, build_cronjob};
    use k8s_openapi::api::core::v1::Secret;
    use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    use kube::ResourceExt;
    use std::sync::Arc;

    fn build_secret() -> Secret {
        Secret {
            metadata: ObjectMeta {
                ..ObjectMeta::default()
            },
            ..Secret::default()
        }
    }

    #[cfg(test)]
    fn test_build_cronjob() {
        let secret = Arc::from(build_secret());
        let cronjob = build_cronjob(&secret, "test-secret", "0");
        assert_eq!(build_cron_name(&*Arc::new(secret), "0"), cronjob.name_any())
    }
}
