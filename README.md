![Logo for rūnō](assets/logo.png)

# rūnō - A Secret Generator for Kubernetes

While navigating the great ocean of Kubernetes and spinning up cluster after cluster and environment after environment, a little help is very welcome. rūnō is helping you with the in-cluster generation of secret strings. If no external storage of secrets is needed or desired, and you only want to have a random string for e.g. a database password, a user password or a replica set key, this controller is maybe helpful to you. 
It supports:
- Automatic field generation for Kubernetes secrets
- Configuration via annotations
- Definition of value length, pattern and/or charset per field
- Rotation of secret values based on cron schedules


## **Important**:
> rūnō is a personal project! It's important for me to mention that I'm the only maintainer at the moment and because 
> humans make mistakes (a lot of mistakes), please be careful when using it for production environments.


## Annotations

v1.secret.runo.rocks/generate
----
```
apiVersion: v1
kind: Secret
metadata:
  name: example-secret
  annotations:
    v1.secret.runo.rocks/generate-${ID}=${FIELD_NAME} # Example: password
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
  annotations:
    v1.secret.runo.rocks/generate-${ID}=${FIELD_NAME} # Example: password
    v1.secret.runo.rocks/length-${ID}=${LENGTH_OF_THE_VALUE} # Example: 10
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
  annotations:
    v1.secret.runo.rocks/generate-${ID}=${FIELD_NAME} # Example: replica_set_key
    v1.secret.runo.rocks/length-${ID}=${LENGTH_OF_THE_VALUE} # Example: 5
    v1.secret.runo.rocks/charset-${ID}=${CHARSET} # Example: abcd
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
  annotations:
    v1.secret.runo.rocks/generate-${ID}=${FIELD_NAME} # Example: password
    v1.secret.runo.rocks/length-${ID}=${LENGTH_OF_THE_VALUE} # Example: 5
    v1.secret.runo.rocks/pattern-${ID}=${CHARSET} # Example: abcd
  type: Opaque
  data:
```
A more powerful approach is the use of a regular expression to specify the pattern of the field. The generator is using the [rand_regex](https://crates.io/crates/rand_regex) crate for the actual generation.

v1.secret.runo.rocks/regeneration-cron
----
```
apiVersion: v1
kind: Secret
metadata:
  name: example-secret
  annotations:
    v1.secret.runo.rocks/generate-${ID}=${FIELD_NAME} # Example: password
    v1.secret.runo.rocks/length-${ID}=${LENGTH_OF_THE_VALUE} # Example: 5
    v1.secret.runo.rocks/pattern-${ID}=${CHARSET} # Example: abcd
    v1.secret.runo.rocks/regeneration-cron-${ID}=${CRON_SPEC}
  type: Opaque
  data:
```
Sometimes its helpful or even necessary to rotate secrets after some time. rūnō helps you with that by regenerating fields based on Cron specifications. In the background, rūnō makes use of Kubernetes CronJobs, so you can just use the regular Kubernetes Cron pattern and rūnō takes care of everything else. 

***Please note*** that not all use cases or applications support secret rotation. Please check carefully before using this feature. There is no history of field values and nobody wants to be locked-out of a production database because of that.

## Deployment

Please deploy rūnō via the [available Helm chart](https://github.com/AljoschaP/runo-helm-chart).

## Alternatives

- [kubernetes-secrets-generator](https://github.com/mittwald/kubernetes-secret-generator): A secrets generator written by the awesome guys of Mittwald. If you want to create SSH key pairs or Ingress Basic Auth, try the kubernetes-secrets-generator. 
- [secretize](https://github.com/bbl/secretize): A plugin for kubectl, which generates Kubernetes secrets based on different data sources (e.g. AWS Secrets Manager, Azure Vault, Hashicorp Vault). If you don't want to deploy a component into your cluster, this one.
