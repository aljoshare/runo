# Feature Specification: Secret Field Cloning

**Feature Branch**: `002-field-cloning`

**Created**: 2025-05-08

**Status**: Ratified / Implemented

**Input**: User description: "Support clone-from-${ID} annotation to duplicate the generated value of one secret field to another field within the same Secret, forbidding cyclic or transitive clones."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Duplicate Generated Field in Same Secret (Priority: P1)

As a Kubernetes application author, I want a generated secret string to be duplicated into multiple secret keys under different names (e.g. `password` and `DATABASE_PASSWORD`), so that different microservices or libraries can consume the same credential using their preferred key names.

**Why this priority**: Solves a major use case without requiring external sync operators.

**Independent Test**: Define field `0` as `password` and field `1` as `db_password` with `clone-from-1: "0"`. Verify `Secret.data["db_password"]` contains the exact same bytes as `Secret.data["password"]`.

**Acceptance Scenarios**:

1. **Given** field ID `0` (`generate-0: "password"`) and field ID `1` (`generate-1: "db_password"`, `clone-from-1: "0"`),
   **When** runo reconciles,
   **Then** `needs_clone` returns `true` for field `1`, field `0` is generated first, and field `1` receives the identical byte payload.

---

### User Story 2 - Prevent Transitive / Chained Cloning (Priority: P2)

As a cluster operator, I want runo to prevent cloning from an already cloned field (e.g. `2` clones `1` which clones `0`), so that dependency resolution remains simple, deterministic, and free of circular reference deadlocks.

**Why this priority**: Protects against cyclic dependency crashes and non-deterministic reconciliation loops.

**Independent Test**: Configure field `2` with `clone-from-2: "1"` where field `1` has `clone-from-1: "0"`. Verify runo logs an error and aborts the clone rather than executing transitive cloning.

**Acceptance Scenarios**:

1. **Given** field `1` has `clone-from-1: "0"` and field `2` has `clone-from-2: "1"`,
   **When** runo reconciles field `2`,
   **Then** `should_clone_already_cloned_field` returns `true`, an error is logged, and reconciliation returns `DataUpdateError`.

---

### User Story 3 - Robust Handling of Missing or Unready Source (Priority: P3)

As a developer, I want clear errors when referencing a non-existent or unpopulated clone source ID.

**Why this priority**: Improves developer ergonomics and debugging speed for annotation typos.

**Independent Test**: Set `clone-from-1: "99"` where ID `99` does not exist. Verify runo logs an error and does not panic.

**Acceptance Scenarios**:

1. **Given** field `1` specifies `clone-from-1: "99"` and ID `99` has no `generate-99` annotation,
   **When** runo executes reconciliation,
   **Then** an error is logged stating "Can't clone field! No annotation for field with id 99".

---

### Edge Cases

- **Self-Cloning**: A field targeting itself (`clone-from-0: "0"`). Must be blocked as invalid source data.
- **Empty Source Data**: Source field annotation exists, but source data field is currently missing or empty in `Secret.data`. Must return error and avoid writing an empty string.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST support `v1.secret.runo.rocks/clone-from-${ID}` referencing another field's ID.
- **FR-002**: `needs_clone(secret, id)` MUST return `true` if `clone-from-${ID}` is present and field is not paused.
- **FR-003**: System MUST skip random generation for field `${ID}` when `needs_clone` is `true`.
- **FR-004**: System MUST check if source field is itself cloned and reject transitive cloning.
- **FR-005**: Cloned values MUST be copied verbatim as `ByteString`.

### Key Entities

- **V1Annotation::CloneFrom**: Annotation enum mapping to `v1.secret.runo.rocks/clone-from-${ID}`.
- **ByteString**: The raw byte payload copied from the source secret data entry.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Cloned data keys are identical byte-for-byte to source keys.
- **SC-002**: Zero circular reference deadlocks.
- **SC-003**: Unit and integration tests cover happy path, chained clone rejection, and missing source error handling.
