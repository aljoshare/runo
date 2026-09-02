# Feature Specification: Pre-existing Field Protection and Force Overwrite

**Feature Branch**: `004-force-overwrite`

**Created**: 2025-02-04

**Status**: Ratified / Implemented

**Input**: User description: "Protect pre-existing Secret data by ignoring existing fields by default, with an opt-in force-overwrite-${ID} annotation to explicitly authorize replacements."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Protect Existing Secret Values (Priority: P1)

As a Kubernetes operator onboarding an existing production secret to runo management, I want runo to NOT overwrite fields that already have values, so that existing credentials are never accidentally destroyed.

**Why this priority**: Critical data loss prevention. Safety-first default behavior.

**Independent Test**: Create a Kubernetes `Secret` with existing key `password: "supersecret"`. Add runo managed label and annotations. Verify runo does not overwrite `password`.

**Acceptance Scenarios**:

1. **Given** a `Secret` with `data.password` already populated,
   **When** runo reconciles the secret with `generate-0: "password"`,
   **Then** runo detects the key already exists and skips generation.

---

### User Story 2 - Explicit Force-Overwrite Opt-In (Priority: P2)

As a developer who intentionally wants runo to replace an existing field value, I want to set `v1.secret.runo.rocks/force-overwrite-${ID}: "true"` to explicitly authorize regeneration.

**Why this priority**: Gives operators intentional, explicit control when replacement is required.

**Independent Test**: Set `force-overwrite-0: "true"` on an existing key. Verify runo generates a fresh value and replaces the existing entry.

**Acceptance Scenarios**:

1. **Given** a `Secret` with pre-existing `data.password` and annotation `v1.secret.runo.rocks/force-overwrite-0: "true"`,
   **When** runo reconciles,
   **Then** runo overwrites `data.password` with a newly generated random value.

---

### Edge Cases

- **Boolean String Parsing**: `force-overwrite` must strictly evaluate `"true"`. Values like `"false"`, `""`, or `"1"` must not trigger overwrite.
- **Multi-field Granularity**: In a secret with multiple fields, `force-overwrite` on field `0` must not affect field `1`.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST inspect `Secret.data` before generating a field.
- **FR-002**: System MUST skip generation if the target field key already exists in `Secret.data` and force-overwrite is not enabled.
- **FR-003**: System MUST support `v1.secret.runo.rocks/force-overwrite-${ID}` to override field protection.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Zero accidental overwrites of existing Kubernetes secret keys.
- **SC-002**: Clean opt-in replacement when `force-overwrite-${ID}: "true"` is configured.
