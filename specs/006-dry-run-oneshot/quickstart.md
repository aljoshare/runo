# Quickstart & Validation Guide: Controller Execution Modes

This guide provides runnable scenarios to validate `--dry-run` and `--one-shot` execution modes end-to-end.

## Prerequisites

- Local Kubernetes cluster (e.g. `kind` or `minikube`) or simulated test environment.
- `runo` binary compiled (`cargo build`).
- `kubectl` configured to target the test cluster.

---

## Validation Scenarios

### Scenario 1: One-Shot Mode Execution

Prove that `runo --one-shot` performs a single reconciliation cycle and terminates cleanly with exit code 0.

#### Setup

1. Create a test namespace and managed secret template:
   ```bash
   kubectl create namespace runo-test
   kubectl apply -n runo-test -f - <<EOF
   apiVersion: v1
   kind: Secret
   metadata:
     name: oneshot-sample
     labels:
       v1.secret.runo.rocks/managed: "true"
     annotations:
       v1.secret.runo.rocks/length-0: "16"
       v1.secret.runo.rocks/key-0: "API_TOKEN"
   data: {}
   EOF
   ```

#### Run

```bash
runo --one-shot
```

#### Expected Outcome

- Process logs execution in one-shot mode:
  ```text
  Running runo in one-shot mode.
  reconcile request: oneshot-sample
  Secret updated successfully!
  ```
- Process exits automatically with status code `0`.
- Verified via `echo $?` returning `0`.
- Secret `oneshot-sample` contains generated key `API_TOKEN`.

---

### Scenario 2: Dry-Run Mode Execution

Prove that `runo --dry-run` logs planned changes without writing modifications to Kubernetes.

#### Setup

1. Create an unpopulated managed secret:
   ```bash
   kubectl apply -n runo-test -f - <<EOF
   apiVersion: v1
   kind: Secret
   metadata:
     name: dryrun-sample
     labels:
       v1.secret.runo.rocks/managed: "true"
     annotations:
       v1.secret.runo.rocks/length-0: "32"
       v1.secret.runo.rocks/key-0: "PASSWORD"
   data: {}
   EOF
   ```

#### Run

```bash
runo --dry-run --one-shot
```

#### Expected Outcome

- Process logs dry-run execution:
  ```text
  Running runo in dry-run mode!
  reconcile request: dryrun-sample
  ```
- Kubernetes Secret `dryrun-sample` data remains empty (`data: {}`), proving zero mutating writes were committed.
- No plaintext passwords appear in the logs.
- Process terminates cleanly with exit code `0`.

---

### Scenario 3: Automated CLI Contract Tests

Run the deterministic CLI argument validation test suite via `cargo test`:

```bash
cargo test --test test_main
```

#### Expected Outcome

- `test_main::dry_run` passes.
- New test `test_main::one_shot` passes (verifies `--one-shot` flag accepted).
- New test `test_main::combined_dry_run_one_shot` passes.
