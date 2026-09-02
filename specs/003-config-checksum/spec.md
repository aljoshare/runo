# Feature Specification: Configuration Drift Checksum and Automatic Regeneration

**Feature Branch**: `003-config-checksum`

**Created**: 2024-11-14

**Status**: Ratified / Implemented

**Input**: User description: "Track configuration checksum (SHA256) per field ID and automatically trigger secret regeneration when length, charset, or pattern annotations change."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Detect Configuration Drift (Priority: P1)

As a developer updating secret requirements (e.g. increasing password length from 16 to 32 characters or enforcing a stricter regex pattern), I want runo to automatically detect that the configuration annotations changed and regenerate the secret to match the new spec.

**Why this priority**: Essential declarative behavior. Ensures Kubernetes Secret contents continuously match the desired state declared in annotations.

**Independent Test**: Create a secret with `length-0: "16"`. After generation, update annotation to `length-0: "32"`. Verify runo regenerates a 32-character secret on the next reconciliation.

**Acceptance Scenarios**:

1. **Given** a generated secret with `generated-with-checksum-0` matching the original parameters,
   **When** an operator changes `v1.secret.runo.rocks/length-0` from `16` to `32`,
   **Then** the recalculated `config-checksum-0` diverges from `generated-with-checksum-0`, and `needs_generation` returns `true`.

---

### User Story 2 - Prevent Infinite Re-reconciliation Loops (Priority: P2)

As a cluster administrator, I want runo to avoid self-triggering updates by ignoring internal checksum annotations during drift comparison.

**Why this priority**: Prevents CPU spikes and etcd write thrashing caused by controller feedback loops.

**Independent Test**: Verify that updating `generated-with-checksum` annotation does not trigger a re-generation event.

**Acceptance Scenarios**:

1. **Given** a secret where runo updates `generated-with-checksum-0` to match `config-checksum-0`,
   **When** the watch event fires for the metadata change,
   **Then** `needs_generation` returns `false` and no API update is issued.

---

### Edge Cases

- **Pre-existing Secrets without Checksum**: If `generated-with-checksum` annotation is missing (legacy secret), runo must record the checksum without unexpectedly regenerating the secret unless forced.
- **Checksum Collision**: Handled by using full SHA256 cryptographic hashing over concatenated configuration parameters.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST compute SHA256 hash over `length`, `pattern`, and `charset` for each field ID.
- **FR-002**: System MUST store current configuration hash in `v1.secret.runo.rocks/config-checksum-${ID}`.
- **FR-003**: System MUST record the hash used at generation time in `v1.secret.runo.rocks/generated-with-checksum-${ID}`.
- **FR-004**: System MUST trigger regeneration if `config-checksum` does not match `generated-with-checksum`.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Changing any generation annotation automatically results in a new compliant secret value.
- **SC-002**: Idle secrets produce zero unnecessary update events or reconciliation loops.
