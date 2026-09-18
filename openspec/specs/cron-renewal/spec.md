# Cron Renewal

## Purpose
The cron renewal capability enables automated periodic rotation of secret credentials by scheduling and managing Kubernetes CronJobs that trigger runo reconciliation cycles.

## Requirements

### Requirement: Automated Rotation Scheduling
The system SHALL manage Kubernetes CronJobs based on `v1.secret.runo.rocks/renewal-cron-${ID}` schedules to trigger credential renewals.

#### Scenario: Schedule field rotation
- **WHEN** a secret field specifies a valid cron expression in `renewal-cron-${ID}`
- **THEN** runo creates or updates a Kubernetes CronJob targeting that secret field

### Requirement: CronJob Cleanup
The system MUST delete associated CronJobs when cron renewal annotations are removed or disabled.

#### Scenario: Remove renewal schedule
- **WHEN** `renewal-cron-${ID}` is deleted or disabled on a secret field
- **THEN** runo deletes the corresponding Kubernetes CronJob resource
