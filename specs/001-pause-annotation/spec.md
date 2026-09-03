# Feature Specification: Pause Annotation for Managed Secret Fields

**Feature Branch**: `001-pause-annotation`

**Created**: 2026-05-02

**Status**: Ratified / Implemented

**Input**: User description: "Add pause annotation (v1.secret.runo.rocks/pause-${ID}) to temporarily suspend reconciliation, generation, cloning, and renewal for specific secret fields."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Suspend Generation on Target Field (Priority: P1)

As a Kubernetes platform engineer performing cluster migrations or template refactoring, I want to pause runo from generating or altering a specific secret field so that existing application secrets remain stable and are not overwritten during maintenance.

**Why this priority**: Core safety requirement. Prevents unintended mutations to live application credentials.

**Independent Test**: Add `v1.secret.runo.rocks/pause-0: "true"` to an ungenerated secret field. Verify that runo reconciles the secret but leaves field ID `0` ungenerated and untouched in `Secret.data`.

**Acceptance Scenarios**:

1. **Given** a managed Kubernetes `Secret` with `v1.secret.runo.rocks/generate-0: "password"` and `v1.secret.runo.rocks/pause-0: "true"`,
   **When** runo executes reconciliation,
   **Then** `needs_generation` returns `false` for field `0`, and no data is created for `"password"`.

2. **Given** a secret where field `0` is paused and field `1` (`v1.secret.runo.rocks/generate-1: "api_key"`) is not paused,
   **When** runo executes reconciliation,
   **Then** field `1` is generated normally while field `0` remains untouched.

---

### User Story 2 - Suspend Cron-Based Renewal When Paused (Priority: P2)

As an operator managing expiring credentials, I want to prevent scheduled cron renewals from executing when a field is paused so that I can hold key rotations during planned service maintenance windows.

**Why this priority**: Essential operational control to prevent scheduled rotation disruptions during incidents or freezes.

**Independent Test**: Configure a field with expired `renewal-cron` and `pause: "true"`. Verify that the controller reconciliation loop skips renewal and emits a debug log without changing the secret value.

**Acceptance Scenarios**:

1. **Given** a managed secret field with `v1.secret.runo.rocks/renewal-0: "true"`, an elapsed `renewal-cron-0`, and `v1.secret.runo.rocks/pause-0: "true"`,
   **When** runo checks if renewal is required,
   **Then** `needs_renewal` returns `false` and the existing `Secret.data["password"]` and `generated-at-0` annotations remain unchanged.

---

### User Story 3 - Suspend Field Cloning When Paused (Priority: P3)

As a developer using `clone-from` to synchronize multiple secret keys, I want the pause annotation to halt cloning into the target field if the target field is marked as paused.

**Why this priority**: Ensures consistency across all secret mutation pathways (generation, renewal, cloning).

**Independent Test**: Create fields `0` and `1`, where field `1` has `clone-from-1: "0"` and `pause-1: "true"`. Verify field `1` is not modified.

**Acceptance Scenarios**:

1. **Given** field `0` generated and field `1` configured with `clone-from-1: "0"` and `pause-1: "true"`,
   **When** runo reconciles,
   **Then** `needs_clone` returns `false` for field `1`.

---

### Edge Cases

- **Malformed Boolean Values**: If `v1.secret.runo.rocks/pause-${ID}` contains invalid text (e.g. `"yes"`, `"1"`, or empty string), it must safely parse and default to `false` (unpaused) without raising a panic or error.
- **Missing Pause Annotation**: If the annotation does not exist on the Secret, `is_paused` must return `false`.
- **Unpausing a Field**: When `pause-${ID}` is changed from `"true"` to `"false"` or removed, the next reconciliation cycle must resume normal generation, renewal, and cloning workflows immediately.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST recognize annotation `v1.secret.runo.rocks/pause-${ID}`.
- **FR-002**: `is_paused(secret, id)` MUST return `true` if and only if the annotation value parses strictly to boolean `true`.
- **FR-003**: `needs_generation(secret, id)` MUST return `false` if `is_paused(secret, id)` is `true`.
- **FR-004**: `needs_renewal(secret, id)` MUST return `false` if `is_paused(secret, id)` is `true`.
- **FR-005**: `needs_clone(secret, id)` MUST return `false` if `is_paused(secret, id)` is `true`.
- **FR-006**: The controller MUST log a structured debug message when skipping operations on paused fields.

### Key Entities

- **V1Annotation::Pause**: Represents the annotation enum variant mapping to `v1.secret.runo.rocks/pause` and formatted as `v1.secret.runo.rocks/pause-${ID}`.
- **Secret Resource**: Kubernetes `v1/Secret` object containing metadata annotations and data payload.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Zero unintended secret mutations occur on fields marked with `pause-${ID}: "true"`.
- **SC-002**: 100% test coverage for pause permutations (unit tests covering `is_paused_true`, `is_paused_false`, `needs_no_generation_when_paused`, `needs_no_renewal_when_paused`, and `needs_no_clone_when_target_paused`).
- **SC-003**: Documentation updated in `README.md` explaining syntax and migration use cases.

## Assumptions

- The pause annotation operates on a per-field `${ID}` level, not whole-secret level, allowing granular control.
- Existing Kubernetes secrets without this annotation continue to behave with default unpaused behavior.
