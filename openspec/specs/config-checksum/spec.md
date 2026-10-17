# Configuration Checksum

## Purpose
The configuration checksum capability tracks SHA-256 digests of field generation parameters to detect configuration drift and automatically trigger secret regeneration when annotations change.

## Requirements

### Requirement: Configuration Drift Detection
The system SHALL compute a SHA-256 hash over generation parameters (`generate`, `length`, `charset`, `pattern`, `clone-from`) and trigger regeneration when current annotations drift from the recorded checksum.

#### Scenario: Detect parameter updates
- **WHEN** an operator updates generation configuration annotations such as length or pattern
- **THEN** runo detects checksum mismatch and regenerates the secret value in a single reconciliation pass

### Requirement: Feedback Loop Prevention
The system MUST exclude operational annotations, status annotations, and foreign annotations from the checksum calculation.

#### Scenario: Non-generation annotation changes
- **WHEN** operational annotations like pause or force-overwrite change, or metadata checksum annotations update
- **THEN** configuration checksum remains stable and does not trigger spurious regeneration loops
