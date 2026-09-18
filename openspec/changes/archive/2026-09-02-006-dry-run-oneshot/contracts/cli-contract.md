# CLI Contract: Controller Execution Modes

## Interface: Command-Line Interface (`runo`)

### Command Usage

```bash
runo [OPTIONS]
```

### Flags and Options

| Option | Flag | Type | Default | Description |
| :--- | :--- | :--- | :--- | :--- |
| `--dry-run` | Optional | Boolean | `false` | When present, runs the controller in dry-run mode. Logs actions at `INFO` level without committing modifications to Kubernetes resources. |
| `--one-shot` | Optional | Boolean | `false` | When present, executes a single reconciliation cycle over all managed secrets and terminates upon completion. |
| `--mode` | Optional | String | `"reconciliation"` | *(Deprecated)* Explicit execution mode name (`"reconciliation"` or `"one-shot"`). Retained for backwards compatibility. |
| `--http-port` | Optional | `u16` | `8080` | Port for the HTTP health/metrics server (daemon mode only). |
| `--requeue-duration` | Optional | `u64` | `300` | Requeue interval in seconds for reconciliation cycles (daemon mode only). |
| `--help`, `-h` | Optional | Flag | N/A | Prints help information and exits with code 0. |
| `--version`, `-V` | Optional | Flag | N/A | Prints version information and exits with code 0. |

### Exit Codes

| Exit Code | Condition |
| :--- | :--- |
| `0` | Successful execution and termination (one-shot cycle completed cleanly, `--help`, or `--version`). |
| `1` | Command-line argument parsing error, fatal cluster connection failure, or unhandled reconciliation error. |
| `130 / 143` | Interrupted by `SIGINT` or `SIGTERM` in daemon mode. |

### Behavioral Contracts

1. **Combined Mode (`runo --dry-run --one-shot`)**:
   - Executes a single pass listing managed secrets with `v1.secret.runo.rocks/managed=true`.
   - Simulates secret generation and updates without committing mutations to etcd.
   - Logs simulated actions at `INFO` level.
   - Terminates with exit code `0` upon completing the pass.

2. **Daemon Mode (`runo`)**:
   - Starts HTTP server on configured `--http-port`.
   - Starts long-lived watcher on secrets matching managed label selector.
   - Runs until terminated by OS signal (`SIGINT`, `SIGTERM`).

3. **Dry-Run Logging Guard**:
   - Log entries MUST NEVER print secret values in plaintext.
   - Example expected log event:
     ```text
     {"level":"INFO","message":"[DRY-RUN] Would generate and apply secret","secret":"default/app-db-creds"}
     ```
