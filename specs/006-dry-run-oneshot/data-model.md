# Data Model: Controller Execution Modes (Dry-Run and One-Shot)

## Core Entities

### 1. MainArgs (CLI Input Entity)

Represents user-supplied command line arguments parsed via `clap`.

| Field | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `http_port` | `u16` | `8080` | Port for the HTTP health and readiness server |
| `dry_run` | `bool` | `false` | When true, executes dry-run without mutating Kubernetes objects |
| `one_shot` | `bool` | `false` | When true, executes single reconciliation pass over secrets and exits |
| `mode` | `Option<String>` | `None` | Deprecated execution mode (`reconciliation` vs `one-shot`) for backwards compatibility |
| `requeue_duration` | `u64` | `300` | Reconciliation requeue interval in seconds for daemon mode |

#### Validation & Derivation Rules

- If `one_shot == true` OR `mode == Some("one-shot")`: Resolved execution mode is `ExecutionMode::OneShot`.
- If `one_shot == false` AND `mode != Some("one-shot")`: Resolved execution mode is `ExecutionMode::Daemon`.
- If `dry_run == true`: Sets `K8s.dry_run = true` across all API calls and logs planned mutations at `INFO` level.

---

### 2. ExecutionMode (Runtime Controller State)

Defines the lifecycle pattern under which the controller processes Kubernetes resources.

```rust
pub enum ExecutionMode {
    /// Long-running daemon listening to Kubernetes watcher events and running HTTP health probes.
    Daemon {
        http_port: u16,
        requeue_duration: u64,
    },
    /// Batch/Job execution performing a single reconciliation pass over managed secrets and terminating.
    OneShot,
}
```

#### Lifecycle Transitions

```text
[ Process Start ]
       │
       ▼
[ Parse CLI Args ]
       │
       ├──> (one_shot = true) ───────> [ ExecutionMode::OneShot ]
       │                                       │
       │                                       ▼
       │                                [ List Managed Secrets ]
       │                                       │
       │                                       ▼
       │                                [ Reconcile Each Secret ]
       │                                       │
       │                                       ▼
       │                                [ Terminate: exit(0) or error ]
       │
       └──> (one_shot = false) ──────> [ ExecutionMode::Daemon ]
                                               │
                                               ▼
                                   [ Spawn HTTP Health Server ]
                                               │
                                               ▼
                                   [ Start Controller Watcher Loop ]
```

---

### 3. K8s (API Client & Mutation Guard)

Controls mutation parameters for all Kubernetes API requests.

| Field | Type | Description |
| :--- | :--- | :--- |
| `dry_run` | `bool` | When `true`, all `PatchParams` and `PostParams` have `dry_run = true` |

#### Methods

- `build(dry_run: bool) -> K8s`: Initializes K8s client helper with dry-run state.
- `get_patch_params(self) -> PatchParams`: Generates patch parameters with `dry_run` flag.
- `get_post_params(self) -> PostParams`: Generates post parameters with `dry_run` flag.

---

### 4. SecretReconciliationOutcome

Represents the result of evaluating a single Kubernetes `v1/Secret`.

| Field | Type | Description |
| :--- | :--- | :--- |
| `secret_name` | `String` | Name of the managed Secret |
| `namespace` | `String` | Namespace of the managed Secret |
| `action_taken` | `ActionType` | Generated, Renewed, Unmodified, or DryRunSimulated |
| `status` | `Result<(), Error>` | Success or failure of reconciliation |
