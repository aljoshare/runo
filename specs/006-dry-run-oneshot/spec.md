# Feature Specification: Controller Execution Modes (Dry-Run and One-Shot)

**Feature Branch**: `006-dry-run-oneshot`

**Created**: 2023-07-11

**Status**: Ratified / Implemented

**Input**: User description: "Add CLI flags --dry-run to log actions without mutating Kubernetes objects, and --one-shot to execute a single reconciliation cycle and exit for Kubernetes Jobs."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - One-Shot Mode for Kubernetes Jobs (Priority: P1)

As a DevOps engineer, I want to run runo as a Kubernetes Job or initContainer that reconciles all secrets once and exits with code 0, rather than running as a persistent daemon.

**Why this priority**: Enables batch workflows, GitOps sync hooks, and resource-constrained environments.

**Independent Test**: Execute `runo --one-shot`. Verify the process runs through the secret list, reconciles, and terminates cleanly.

**Acceptance Scenarios**:

1. **Given** runo is started with `--one-shot`,
   **When** reconciliation of all discovered managed secrets completes,
   **Then** the process exits with status code `0`.

---

### User Story 2 - Dry-Run Mode for Verification (Priority: P2)

As an operator preparing a cluster migration, I want to simulate runo's execution with `--dry-run` to preview all proposed secret modifications in the logs without writing to Kubernetes etcd.

**Why this priority**: Eliminates risk when validating annotations and configurations on existing clusters.

**Independent Test**: Execute `runo --dry-run`. Verify logs indicate proposed actions while Kubernetes Secret resources remain untouched.

**Acceptance Scenarios**:

1. **Given** runo is started with `--dry-run`,
   **When** a secret requires generation or renewal,
   **Then** the action is logged at `INFO` level, and no `kube::Api::replace` or `patch` call is executed.

---

### Edge Cases

- **Combined Flags (`--dry-run --one-shot`)**: Both flags can be used simultaneously to perform a single-pass read-only inspection and exit.
- **Error in One-Shot**: If an irrecoverable error occurs during one-shot execution, process must exit with non-zero exit code.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: CLI MUST accept `--dry-run` flag via `clap`.
- **FR-002**: CLI MUST accept `--one-shot` flag via `clap`.
- **FR-003**: In dry-run mode, all mutating Kubernetes API requests MUST be bypassed.
- **FR-004**: In one-shot mode, the controller MUST terminate after the initial list-reconciliation cycle.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Zero mutating Kubernetes requests executed under `--dry-run`.
- **SC-002**: `--one-shot` process terminates cleanly with appropriate exit code.
