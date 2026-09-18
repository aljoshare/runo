# Force Overwrite

## Purpose
The force overwrite capability allows operators to command runo to overwrite existing, manually altered, or unmanaged secret data keys during reconciliation.

## Requirements

### Requirement: Overwrite Existing Secret Keys
The system SHALL overwrite pre-existing secret keys when `v1.secret.runo.rocks/force-overwrite-${ID}` is set to `"true"`.

#### Scenario: Overwrite unmanaged or drifted key
- **WHEN** a secret key exists prior to runo management and `force-overwrite` is enabled
- **THEN** runo replaces the existing value with newly generated secret data

### Requirement: Safe Preservation Without Overwrite
The system MUST preserve existing keys when `force-overwrite` is false or omitted, avoiding accidental data loss.

#### Scenario: Default non-destructive behavior
- **WHEN** an existing key is present in Secret data and `force-overwrite` is not true
- **THEN** runo skips overwriting the key and logs an informational warning
