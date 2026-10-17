# Field Cloning

## Purpose
The field cloning capability allows secret fields within the same managed Kubernetes Secret to duplicate values from another field, supporting multi-key synchronization without regenerating redundant random data.

## Requirements

### Requirement: Synchronized Field Cloning
The system SHALL clone the generated value from a source field ID to a target field ID within the same Secret.

#### Scenario: Clone generated secret value
- **WHEN** a secret field specifies `v1.secret.runo.rocks/clone-from-${TARGET_ID}: "${SOURCE_ID}"`
- **THEN** runo copies the value generated for source field into the target field

### Requirement: Source Dependency Ordering
The system MUST resolve and generate source fields prior to evaluating dependent clone fields.

#### Scenario: Unready source field
- **WHEN** source field is not yet generated in the current reconciliation cycle
- **THEN** target field waits for source generation or requeues safely without error
