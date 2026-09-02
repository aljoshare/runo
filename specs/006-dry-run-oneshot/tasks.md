# Tasks: Controller Execution Modes (Dry-Run and One-Shot)

**Branch**: `006-dry-run-oneshot` | **Spec**: [specs/006-dry-run-oneshot/spec.md](spec.md) | **Plan**: [specs/006-dry-run-oneshot/plan.md](plan.md)

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Review existing CLI and runtime entrypoints to prepare for execution mode additions.

- [ ] T001 Review existing CLI arguments and execution flow in src/main.rs
- [ ] T002 [P] Inspect test harness configuration for CLI testing in tests/test_main.rs

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be verified before user story implementation begins.

**⚠️ CRITICAL**: All story work depends on foundational parameter and mode handling.

- [ ] T003 Ensure K8s struct in src/k8s.rs correctly exposes dry_run configuration in PatchParams and PostParams
- [ ] T004 [P] Define ExecutionMode resolution logic mapping CLI flags (one_shot, mode) in src/main.rs

**Checkpoint**: Foundation ready - user story implementation can now begin.

---

## Phase 3: User Story 1 - One-Shot Mode for Kubernetes Jobs (Priority: P1) 🎯 MVP

**Goal**: Enable running runo as a one-shot batch job that reconciles all managed secrets once and terminates cleanly with exit code 0.

**Independent Test**: Run `runo --one-shot` against managed secrets and verify the process reconciles discovered secrets and terminates with exit code 0 without launching a persistent HTTP server.

### Tests for User Story 1

- [ ] T005 [P] [US1] Add CLI argument test for --one-shot flag in tests/test_main.rs
- [ ] T006 [P] [US1] Add integration test asserting process termination in one-shot mode in tests/test_main.rs

### Implementation for User Story 1

- [ ] T007 [US1] Add --one-shot clap argument to MainArgs and deprecate mode in src/main.rs
- [ ] T008 [US1] Update run_one_shot to use managed label selector and return Result in src/reconciler.rs
- [ ] T009 [US1] Wire one-shot mode dispatch in main to bypass actix-web HTTP server and exit cleanly in src/main.rs
- [ ] T010 [US1] Handle errors during one-shot execution to exit with non-zero status on failure in src/main.rs

**Checkpoint**: User Story 1 is fully functional and independently testable as an MVP.

---

## Phase 4: User Story 2 - Dry-Run Mode for Verification (Priority: P2)

**Goal**: Simulate secret reconciliation with `--dry-run` to preview proposed actions in logs at `INFO` level without writing mutations to Kubernetes etcd.

**Independent Test**: Run `runo --dry-run --one-shot` and verify logs indicate proposed actions while Kubernetes Secret resources remain untouched.

### Tests for User Story 2

- [ ] T011 [P] [US2] Add CLI argument test for combined --dry-run and --one-shot in tests/test_main.rs

### Implementation for User Story 2

- [ ] T012 [US2] Add structured INFO logging for simulated secret updates in src/secrets.rs
- [ ] T013 [US2] Audit dry-run logging to ensure zero plaintext secret values are leaked in src/secrets.rs
- [ ] T014 [US2] Verify CronJob creation and replacement respects dry_run params in src/cron.rs

**Checkpoint**: User Stories 1 and 2 are both functional, tested, and verifiable individually and in combination.

---

## Phase 5: Polish & Cross-Cutting Concerns

**Purpose**: Documentation, formatting, quality gates, and final end-to-end scenario validation.

- [ ] T015 [P] Update CLI options documentation in README.md
- [ ] T016 Run formatting and lint checks (cargo fmt and cargo clippy) across src/main.rs and src/reconciler.rs
- [ ] T017 Execute validation scenarios defined in specs/006-dry-run-oneshot/quickstart.md

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately.
- **Foundational (Phase 2)**: Depends on Setup completion - blocks user stories.
- **User Story 1 (Phase 3)**: Depends on Foundational completion. Delivers core MVP.
- **User Story 2 (Phase 4)**: Depends on Foundational completion. Builds on User Story 1 CLI flags.
- **Polish (Phase 5)**: Depends on all user stories being completed.

### User Story Dependencies

- **User Story 1 (P1)**: Independent of User Story 2. Can be implemented and verified standalone.
- **User Story 2 (P2)**: Integrates with CLI flags and execution pathways established in User Story 1.

### Within Each User Story

- Tests must be written and verified before or alongside implementation.
- CLI argument definitions before runtime controller dispatch.
- Core execution loop before error handling and exit code propagation.

### Parallel Opportunities

- T001 and T002 in Setup can be reviewed in parallel.
- T003 and T004 in Foundational can be prepared in parallel.
- Tests T005 and T006 can run in parallel.
- T011 test and T015 README documentation can be developed in parallel with implementation.

---

## Parallel Example: User Story 1

```bash
# Prepare test coverage in parallel:
Task: "Add CLI argument test for --one-shot flag in tests/test_main.rs"
Task: "Add integration test asserting process termination in one-shot mode in tests/test_main.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001, T002).
2. Complete Phase 2: Foundational (T003, T004).
3. Complete Phase 3: User Story 1 (T005-T010).
4. **STOP and VALIDATE**: Test `runo --one-shot` independently.
5. Release/Deploy MVP for batch Kubernetes Jobs.

### Incremental Delivery

1. Setup + Foundational -> Foundation verified.
2. User Story 1 -> One-shot execution verified (MVP).
3. User Story 2 -> Dry-run simulation and safe logging verified.
4. Polish -> Documentation, lints, and quickstart scenarios confirmed.
