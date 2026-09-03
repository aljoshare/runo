# Feature Specification: Periodic Secret Value Rotation via Cron Schedules

**Feature Branch**: `005-cron-renewal`

**Created**: 2023-09-04

**Status**: Ratified / Implemented

**Input**: User description: "Support periodic secret rotation based on cron schedules via renewal-cron-${ID}, recording RFC3339 timestamps in generated-at-${ID}."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Cron-Triggered Secret Rotation (Priority: P1)

As a security engineer, I want secret credentials to rotate automatically on a regular schedule (e.g. monthly or weekly), so that leaked credentials expire automatically without manual intervention.

**Why this priority**: Primary security feature for credential lifecycle compliance.

**Independent Test**: Configure `renewal-0: "true"` and `renewal-cron-0: "* * * * *"` with an expired `generated-at-0` timestamp. Verify secret is renewed and new timestamp written.

**Acceptance Scenarios**:

1. **Given** a secret field with `v1.secret.runo.rocks/renewal-0: "true"` and valid `renewal-cron-0`,
   **When** `Utc::now()` is greater than the next scheduled execution after `generated-at-0`,
   **Then** `needs_renewal` returns `true`, the secret is regenerated, and `generated-at-0` is updated to current UTC timestamp.

---

### User Story 2 - Controller Requeue Scheduling (Priority: P2)

As a cluster operator, I want the controller to sleep and requeue reconciliation exactly when the next rotation is due, so that rotation happens promptly without wasteful busy-polling.

**Why this priority**: Optimizes controller performance and minimizes unnecessary Kubernetes API traffic.

**Independent Test**: Verify controller calculates remaining duration until next cron occurrence and requeues with that duration.

**Acceptance Scenarios**:

1. **Given** a secret scheduled for renewal in 30 minutes,
   **When** the reconciler finishes current pass,
   **Then** it returns `Action::requeue(Duration::from_secs(1800))`.

---

### Edge Cases

- **Invalid Cron Syntax**: Malformed cron string must be logged as an error without crashing the controller.
- **Missing `generated-at`**: If timestamp is missing, assume immediate initial generation and record timestamp.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST support `v1.secret.runo.rocks/renewal-cron-${ID}` supporting standard cron syntax.
- **FR-002**: System MUST record RFC3339 timestamp in `v1.secret.runo.rocks/generated-at-${ID}` upon generation.
- **FR-003**: System MUST calculate next rotation timestamp using the `cron` and `chrono` crates.
- **FR-004**: System MUST recalculate requeue duration to wake up when rotation is due.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Rotation occurs within expected time window of defined cron schedule.
- **SC-002**: `generated-at` timestamp accurately reflects the time of the latest rotation.
