# Research: Controller Execution Modes (Dry-Run and One-Shot)

## Technical Decisions

### Decision 1: CLI Argument Representation for Execution Modes

- **Decision**: Add `#[clap(long, default_value_t = false)] one_shot: bool` alongside the existing `dry_run: bool` flag in `MainArgs`. Deprecate `--mode` by making it optional (`Option<String>`), allowing `--one-shot` to be passed directly as a boolean flag while maintaining backwards compatibility if `--mode one-shot` is passed.
- **Rationale**: 
  - Requirement FR-002 explicitly mandates: `CLI MUST accept --one-shot flag via clap`.
  - FR-001 requires: `CLI MUST accept --dry-run flag via clap`.
  - A boolean flag is idiomatic in Rust CLI tools using `clap` (derive mode).
  - Operators running Kubernetes Jobs can supply `args: ["--one-shot"]` or `args: ["--one-shot", "--dry-run"]` cleanly without positional or string-keyed arguments.
- **Alternatives Considered**:
  - *Keep only `--mode <String>`*: Rejected because it directly violates FR-002 and complicates integration with Kubernetes Job specs.
  - *Remove `--mode` entirely*: Breaking change for existing deployments that might still supply `--mode reconciliation`. Making `--mode` optional or having `--one-shot` take precedence maintains smooth backward compatibility.

---

### Decision 2: Kubernetes API Dry-Run Execution Semantics

- **Decision**: Leverage Kubernetes server-side dry-run via `kube::api::PatchParams { dry_run: true, .. }` and `kube::api::PostParams { dry_run: true, .. }`, supplemented by client-side logging at `INFO` level.
- **Rationale**:
  - `kube-rs` natively supports server-side dry run when `dry_run: true` is passed in `PatchParams` and `PostParams`.
  - Server-side dry run validates the complete request pipeline—including schema validation, mutating webhooks, and validating admission webhooks—without committing changes to etcd storage.
  - Satisfies SC-001 ("Zero mutating Kubernetes requests executed under `--dry-run`").
- **Alternatives Considered**:
  - *Pure client-side short-circuiting (skip calling Kubernetes API altogether)*: While this guarantees no cluster mutations, it fails to validate Kubernetes admission rules and RBAC permissions. However, user scenario acceptance test specifically states: "the action is logged at `INFO` level, and no `kube::Api::replace` or `patch` call is executed". Server-side dry-run submits a request with `dryRun=All`, but client-side bypass is also possible. The existing `src/k8s.rs` already configures `PatchParams.dry_run = true` and `PostParams.dry_run = true`. Ensuring both clear INFO logging and server-side dry-run guarantees safety and accurate preview.

---

### Decision 3: One-Shot Lifecycle and Exit Codes

- **Decision**: In one-shot mode (`--one-shot`):
  1. Skip starting the `actix-web` HTTP health probe server (or do not await it).
  2. Perform a single pass listing and reconciling managed secrets.
  3. Propagate any fatal errors so the process exits with non-zero exit code on failure, and exits cleanly (`std::process::exit(0)` or `Ok(())` in `main`) on success.
- **Rationale**:
  - Kubernetes Jobs or initContainers execute to completion. Running an HTTP server designed for long-lived daemon liveness/readiness probes blocks process termination unless explicitly shut down.
  - Requirement FR-004 and SC-002 mandate that the controller terminates after the initial cycle with appropriate exit code (0 on success, non-zero on failure).
- **Alternatives Considered**:
  - *Start HTTP server in a background task and kill it on exit*: Unnecessary resource consumption and port binding for a transient batch Job; initContainers or Jobs do not require health endpoints.

---

### Decision 4: Secret Selection and Label Filtering in One-Shot Mode

- **Decision**: Update `run_one_shot` to query secrets using `ListParams::default().labels(&labels::get_managed_label())` rather than querying all secrets unfiltered across the cluster.
- **Rationale**:
  - In daemon mode (`run_with_reconciliation`), the watcher explicitly filters by `labels::get_managed_label()` (`v1.secret.runo.rocks/managed=true`).
  - Listing all secrets in a large cluster without label selectors incurs unnecessary memory, latency, and RBAC permission requirements (violating Constitution Principle I and Principle of Least Privilege).
  - Still evaluates each secret through `reconcile()` for consistency.
- **Alternatives Considered**:
  - *Keep `secrets.list(&ListParams::default())` unfiltered*: Inefficient and causes RBAC errors if runo lacks permission to read unmanaged secrets in cluster namespaces.

---

### Decision 5: Cryptographic & Secret Integrity during Logging (Constitution II)

- **Decision**: When logging planned or simulated actions under `--dry-run` at `INFO` level, log only secret metadata (namespace, name, annotation keys, generation/renewal trigger) and never log generated secret payloads, random strings, or raw base64 data.
- **Rationale**:
  - Constitution Principle II explicitly mandates: "Generated secrets must never be logged in plaintext across any `tracing` or error outputs."
  - Dry-run preview needs to communicate *what* secret was evaluated and *that* generation/renewal was simulated, without exposing sensitive values to stdout/stderr.
- **Alternatives Considered**:
  - *Log masked or truncated secret values*: Rejected to eliminate any risk of secret leakage.
