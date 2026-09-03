# Implementation Plan: Controller Execution Modes (Dry-Run and One-Shot)

**Branch**: `006-dry-run-oneshot` | **Date**: 2026-09-02 | **Spec**: [specs/006-dry-run-oneshot/spec.md](spec.md)

**Input**: Feature specification from `specs/006-dry-run-oneshot/spec.md`

## Summary

Implement execution mode controls for runo:
1. `--dry-run` flag: Simulates secret reconciliation without mutating Kubernetes resources (via Kubernetes server-side dry-run and client-side logging at `INFO` level).
2. `--one-shot` flag: Reconciles all managed secrets in a single pass and exits cleanly with code 0 (or non-zero on failure), omitting the long-running HTTP daemon server for use in Kubernetes Jobs and initContainers.

## Technical Context

**Language/Version**: Rust 2021 edition (1.75+)

**Primary Dependencies**: `clap` 4.6.1 (derive), `kube` 4.0.0 (runtime, derive), `k8s-openapi` 0.28.0 (v1_36), `tokio` 1.52.3, `tracing` 0.1.42, `actix-web` 4.12.1, `anyhow` 1.0.102

**Storage**: Kubernetes `v1/Secret` resources via Kubernetes API server (etcd); in-memory runtime

**Testing**: `assert_cmd` 2.2.2 (CLI argument integration tests), `rstest` 0.26.1 (unit tests), `cargo test`

**Target Platform**: Linux container (Kubernetes Pod daemon, initContainer, or batch Job)

**Project Type**: CLI binary & Kubernetes controller

**Performance Goals**: Rapid single-cycle one-shot completion (< 5s for typical secret sets); zero latency overhead for dry-run simulation

**Constraints**: Zero mutating requests in dry-run mode (SC-001); clean termination with exit code 0 on one-shot completion (SC-002); never log plaintext secret values (Constitution II)

**Scale/Scope**: Cluster-wide managed secrets (matching `v1.secret.runo.rocks/managed=true`)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle / Gate | Status | Evaluation |
| :--- | :--- | :--- |
| **I. Kubernetes Native & Controller Safety** | **PASS** | One-shot mode uses Kubernetes API list and declarative reconciliation semantics with managed label filtering. Dry-run leverages Kubernetes server-side dry-run (`PatchParams`/`PostParams`). |
| **II. Cryptographic & Secret Integrity** | **PASS** | Dry-run logs planned generation and updates at `INFO` level without ever exposing plaintext secret keys or generated values in logs. |
| **III. Test-First & Deterministic Verification** | **PASS** | CLI argument parsing for `--dry-run`, `--one-shot`, and combinations validated with `assert_cmd` in `tests/test_main.rs`. |
| **IV. Rust Idioms & Zero-Warning Policy** | **PASS** | Rust 2021 edition, `clap` derive idioms, clean `anyhow` error handling in `main.rs`. |
| **V. Observability & Health Probing** | **PASS** | Daemon mode maintains `actix-web` health server; one-shot mode terminates cleanly without binding HTTP port. Structured telemetry via `tracing`. |
| **Quality Gates (`cargo check`, `fmt`, `clippy`, `test`)** | **PASS** | Code will adhere to all formatting, lint, and test quality gates. |

## Project Structure

### Documentation (this feature)

```text
specs/006-dry-run-oneshot/
├── plan.md              # This file (/speckit-plan output)
├── research.md          # Phase 0 output: technical decisions and rationale
├── data-model.md        # Phase 1 output: CLI arguments and runtime execution model
├── quickstart.md        # Phase 1 output: runnable end-to-end validation scenarios
├── contracts/           # Phase 1 output: CLI interface and behavioral contracts
│   └── cli-contract.md
└── tasks.md             # Phase 2 output (/speckit-tasks output)
```

### Source Code (repository root)

```text
src/
├── annotations.rs
├── config.rs
├── cron.rs
├── errors.rs
├── http.rs
├── k8s.rs               # Dry-run PatchParams and PostParams configuration
├── labels.rs
├── logging.rs
├── main.rs              # Clap CLI argument definition (dry-run, one-shot) and mode dispatch
├── reconciler.rs        # run_one_shot listing and reconciliation loop
└── secrets.rs           # Secret generation and mutation handling

tests/
└── test_main.rs         # CLI argument validation tests using assert_cmd
```

**Structure Decision**: Single binary crate rooted at `src/main.rs`. CLI flags `--dry-run` and `--one-shot` are integrated into `MainArgs` in `src/main.rs`. Mode dispatch routes `--one-shot` execution to `reconciler::run_one_shot(config)`. Parameterized and integration tests reside in `tests/test_main.rs`.

## Complexity Tracking

> *No constitution violations or unjustifiable complexity introduced.*

| Violation | Why Needed | Simpler Alternative Rejected Because |
| :--- | :--- | :--- |
| *None* | N/A | N/A |
