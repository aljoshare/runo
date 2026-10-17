# Controller Execution Modes

## Purpose
The controller execution modes capability provides CLI flags for dry-run simulation and one-shot execution, enabling safe preview of reconciliation actions and batch Job execution.

## Requirements

### Requirement: One-Shot Mode Execution
The system SHALL execute a single reconciliation cycle over all managed secrets and terminate with exit code 0 when `--one-shot` is passed.

#### Scenario: Batch Job reconciliation
- **WHEN** runo is started with `--one-shot`
- **THEN** all discovered managed secrets are reconciled once and the process exits with code 0 without starting a persistent HTTP daemon

### Requirement: Dry-Run Mode Simulation
The system MUST simulate reconciliation without executing mutating Kubernetes API requests when `--dry-run` is passed.

#### Scenario: Preview proposed changes
- **WHEN** runo reconciles managed secrets with `--dry-run`
- **THEN** all planned mutations are logged at INFO level and zero mutating API requests are sent to Kubernetes
