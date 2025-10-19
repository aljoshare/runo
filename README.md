![Logo for rūnō](assets/logo.png)

# rūnō - A Secret Generator for Kubernetes

![GitHub Release](https://img.shields.io/github/v/release/aljoshare/runo?style=flat&logo=github&label=release&color=51aff7)
![GitHub Release Date](https://img.shields.io/github/release-date/aljoshare/runo?display_date=published_at&style=flat&logo=github&label=release%20date&color=51aff7)

![Static Badge](https://img.shields.io/badge/language-grey?logo=rust)
![Static Badge](https://img.shields.io/badge/platform-linux-51aff7?logo=docker)
![Static Badge](https://img.shields.io/badge/arch-amd64-51aff7?logo=docker)
![Static Badge](https://img.shields.io/badge/arch-arm64-51aff7?logo=docker)
[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/aljoshare/runo/badge)](https://scorecard.dev/viewer/?uri=github.com/aljoshare/runo)

While navigating the great ocean of Kubernetes and spinning up cluster after cluster and environment after environment, a little help is very welcome. rūnō is helping you with the in-cluster generation of secret strings. If no external storage of secrets is needed or desired, and you only want to have a random string for e.g. a database password, a user password or a replica set key, this controller is maybe helpful to you. 
It supports:
- Automatic field generation for Kubernetes secrets
- Configuration via annotations
- Definition of value length, pattern and/or charset per field
- Rotation of secret values based on cron schedules


## **Important**:
> rūnō is a personal project! It's important for me to mention that I'm the only maintainer at the moment and because 
> humans make mistakes (a lot of mistakes), please be careful when using it for production environments.

## Labels
In order to denote that runo should take care of this secret, you need to add the following label to each managed secret:
```
apiVersion: v1
kind: Secret
metadata:
  name: example-secret
  labels:
    v1.secret.runo.rocks/managed: "true"
```

## Annotations

v1.secret.runo.rocks/generate
----
```
apiVersion: v1
kind: Secret
metadata:
  name: example-secret
  labels:
    v1.secret.runo.rocks/managed: "true"
  annotations:
    v1.secret.runo.rocks/generate-${ID}: ${FIELD_NAME} # Example: password
type: Opaque
data:
```
Basic annotation for the generation of a field. You can specify the name and the id of the field that should be generated. 

v1.secret.runo.rocks/length
----
```
apiVersion: v1
kind: Secret
metadata:
  name: example-secret
  labels:
    v1.secret.runo.rocks/managed: "true"
  annotations:
    v1.secret.runo.rocks/generate-${ID}: ${FIELD_NAME} # Example: password
    v1.secret.runo.rocks/length-${ID}: ${LENGTH_OF_THE_VALUE} # Example: 10
type: Opaque
data:
```
Because its sometimes necessary to set the length of a secret value explicitly, e.g. min or max requirements for passwords, you are free to specify a length ***> 0 and <= 100***.

v1.secret.runo.rocks/charset
----
```
apiVersion: v1
kind: Secret
metadata:
  name: example-secret
  labels:
    v1.secret.runo.rocks/managed: "true"
  annotations:
    v1.secret.runo.rocks/generate-${ID}: ${FIELD_NAME} # Example: replica_set_key
    v1.secret.runo.rocks/length-${ID}: ${LENGTH_OF_THE_VALUE} # Example: 5
    v1.secret.runo.rocks/charset-${ID}: ${CHARSET} # Example: abcd
type: Opaque
data:
```
If you need to limit the variation of characters, and you don't want to create a regular expression, you can specify a charset and rūnō will create a random string based on the charset for you.

v1.secret.runo.rocks/pattern
----
```
apiVersion: v1
kind: Secret
metadata:
  name: example-secret
  labels:
    v1.secret.runo.rocks/managed: "true"
  annotations:
    v1.secret.runo.rocks/generate-${ID}: ${FIELD_NAME} # Example: password
    v1.secret.runo.rocks/length-${ID}: ${LENGTH_OF_THE_VALUE} # Example: 5
    v1.secret.runo.rocks/pattern-${ID}: ${PATTERN} # Example: [a-zA-Z0-9]
type: Opaque
data:
```
A more powerful approach is the use of a regular expression to specify the pattern of the field. The generator is using the [rand_regex](https://crates.io/crates/rand_regex) crate for the actual generation.

***Please note***: You can't use quantifiers `(e.g. +, ?, *, {1,10})` in the regex pattern.

v1.secret.runo.rocks/renewal-cron
----
```
apiVersion: v1
kind: Secret
metadata:
  name: example-secret
  labels:
    v1.secret.runo.rocks/managed: "true"
  annotations:
    v1.secret.runo.rocks/generate-${ID}: ${FIELD_NAME} # Example: password
    v1.secret.runo.rocks/length-${ID}: ${LENGTH_OF_THE_VALUE} # Example: 5
    v1.secret.runo.rocks/pattern-${ID}: ${CHARSET} # Example: abcd
    v1.secret.runo.rocks/renewal-cron-${ID}: ${CRON_SPEC}
type: Opaque
data:
```
Sometimes its helpful or even necessary to rotate secrets after some time. rūnō helps you with that by regenerating fields based on Cron specifications. In the background, rūnō makes use of Kubernetes CronJobs, so you can just use the regular Kubernetes Cron pattern and rūnō takes care of everything else. 

***Please note*** that not all use cases or applications support secret rotation. Please check carefully before using this feature. There is no history of field values and nobody wants to be locked-out of a production database because of that.

v1.secret.runo.rocks/force-overwrite
----
```
apiVersion: v1
kind: Secret
metadata:
  name: example-secret
  labels:
    v1.secret.runo.rocks/managed: "true"
  annotations:
    v1.secret.runo.rocks/generate-${ID}: ${FIELD_NAME} # Example: password
    v1.secret.runo.rocks/force-overwrite-${ID}: true
type: Opaque
data:
```
Annotation to enforce that runo overwrites a field which is already set, e.g. if you have an existing secret which should be managed afterwards by runo and you want to regenerate a single field. By default, runo ignores fields which are already set.

v1.secret.runo.rocks/clone-from
----
```
apiVersion: v1
kind: Secret
metadata:
  name: example-secret
  labels:
    v1.secret.runo.rocks/managed: "true"
  annotations:
    v1.secret.runo.rocks/generate-${ID1}: ${FIELD_NAME_1} # Example: password
    v1.secret.runo.rocks/generate-${ID2}: ${FIELD_NAME_2} # Example: password-cloned
    v1.secret.runo.rocks/clone-from-${ID2}: ${ID1}
type: Opaque
data:
```
Annotation to instruct runo to clone the value of a generated field to another field in the same secret but with a different name. For example, if you would like to generate a secret for an application where you need the same value multiple times but with different identifiers.

## Deployment

Please deploy rūnō via the [available Helm chart](https://github.com/AljoschaP/runo-helm-chart).

## Alternatives

- [kubernetes-secrets-generator](https://github.com/mittwald/kubernetes-secret-generator): A secrets generator written by the awesome guys of Mittwald. If you want to create SSH key pairs or Ingress Basic Auth, try the kubernetes-secrets-generator. 
- [secretize](https://github.com/bbl/secretize): A plugin for kubectl, which generates Kubernetes secrets based on different data sources (e.g. AWS Secrets Manager, Azure Vault, Hashicorp Vault). If you don't want to deploy a component into your cluster, this one.

## Kubernetes Versions

Because rūnō uses [kube.rs](https://kube.rs), it matches its [compatibility matrix](https://kube.rs/kubernetes-version/), which usually matches the stable channel support of the major cloud providers.
