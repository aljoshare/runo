# rūnō Constitution

## Core Principles

### I. Kubernetes Native & Controller Safety
rūnō operates as an in-cluster Kubernetes controller managing `v1/Secret` resources. Every change to reconciliation loops, watchers, or Kubernetes API operations must handle transient API failures gracefully, respect Kubernetes resource versioning and conflict retries, and adhere to declarative reconciliation semantics.

### II. Cryptographic & Secret Integrity
All generated secret values must maintain strict cryptographic hygiene:
- Random string generation must use secure randomness sources (`rand`, `rand_regex`, `sha2`).
- Generated secrets must never be logged in plaintext across any `tracing` or error outputs.
- Validation on annotation lengths, regex patterns, and rotation schedules must fail safely without corrupting existing Kubernetes secret keys.

### III. Test-First & Deterministic Verification (NON-NEGOTIABLE)
- Changes to generation algorithms, annotation parsers, and reconciliation logic require unit and integration tests.
- Parameterized testing with `rstest` for parser edge cases.
- CLI argument validation covered with `assert_cmd`.
- Performance-critical paths evaluated with `criterion` benchmarks.

### IV. Rust Idioms & Zero-Warning Policy
- Code must compile with the Rust 2021 edition.
- Strict lint compliance: `cargo clippy --all-targets -- -D warnings` must pass cleanly.
- Code style: Must conform to `cargo fmt --check`.
- Error handling: Use `thiserror` for domain-specific library errors and `anyhow` where appropriate in binary entrypoints.

### V. Observability & Health Probing
- Controller runtime exposes structured telemetry via `tracing` and `tracing-subscriber`.
- Health and readiness probes served via `actix-web` must accurately reflect controller state.

## Security & RBAC Standards
- Principle of Least Privilege: Any RBAC adjustments in `rbac/` must request only the minimal permissions required for Secret watching, updating, and event recording.
- Memory handling: Sensitive data representations should minimize unnecessary duplication in memory.

## Development Quality Gates
Before any pull request or merge:
1. `cargo check`
2. `cargo fmt --check`
3. `cargo clippy --all-targets -- -D warnings`
4. `cargo test`

**Version**: 1.0.0 | **Ratified**: 2026-09-01

