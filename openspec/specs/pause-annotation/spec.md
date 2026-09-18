# Pause Annotation

## Purpose
The pause annotation capability enables operators and developers to temporarily suspend reconciliation, secret generation, field cloning, and scheduled renewal for specific secret fields without deleting configuration or modifying active credentials.

## Requirements

### Requirement: Generation Suspension
The system SHALL suspend generation on target secret fields when the pause annotation is set to true.

#### Scenario: Pause generation on field
- **WHEN** a secret has `v1.secret.runo.rocks/pause-${ID}` set to `"true"`
- **THEN** runo skips generation for that field ID and leaves existing data untouched

### Requirement: Cron Renewal Suspension
The system SHALL suspend scheduled cron-based renewals when the field is marked as paused.

#### Scenario: Hold rotation during maintenance
- **WHEN** a secret field with an elapsed renewal cron schedule is marked with `pause: "true"`
- **THEN** runo does not execute rotation and keeps existing secret data intact

### Requirement: Field Cloning Suspension
The system SHALL halt cloning into a target field when the target field is paused.

#### Scenario: Pause target of clone
- **WHEN** a target clone field is marked with `pause: "true"`
- **THEN** runo does not update the target field from the source field
