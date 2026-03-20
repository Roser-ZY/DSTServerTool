## ADDED Requirements

### Requirement: Automatic probe candidates must be verified as real Steam roots

The system MUST validate automatically discovered Steam candidates against both required directory structure and at least one Steam-specific marker before returning the candidate as a detected Steam root.

#### Scenario: Valid macOS Steam root
- **WHEN** automatic probing finds `~/Library/Application Support/Steam` and the directory contains macOS Steam markers such as `registry.vdf`, `Steam.AppBundle`, or `config/libraryfolders.vdf`
- **THEN** the system returns the path as `valid`

#### Scenario: Platform-aware validation rules
- **WHEN** root validation runs on different operating systems
- **THEN** the system applies platform-specific marker rules
- **AND** the system MUST NOT require Windows-only directory markers on macOS

#### Scenario: Unrelated folder resembles part of Steam layout
- **WHEN** automatic probing reaches a non-Steam folder that contains some expected directories but lacks Steam-specific markers
- **THEN** the system MUST reject the folder as a Steam root and continue probing

### Requirement: Automatic probing must return not_found when Steam is absent

If no automatically discovered candidate passes Steam root validation, the system MUST return `not_found` rather than surfacing an invalid guessed root.

#### Scenario: Steam not installed on macOS
- **WHEN** automatic probing runs on macOS and no candidate satisfies Steam root validation
- **THEN** the system returns status `not_found`
- **AND** the system does not expose an unrelated user folder as the detected path

### Requirement: Manual path validation remains actionable

The system MUST continue validating manually provided Steam paths and return explicit validation errors when a user-selected directory is not a Steam root.

#### Scenario: User manually selects non-Steam folder
- **WHEN** a user provides a manual path that lacks required Steam directories or Steam-specific markers
- **THEN** the system returns status `invalid`
- **AND** the validation errors identify the missing Steam markers or directories

#### Scenario: User manually selects macOS Steam root
- **WHEN** a user provides macOS Steam root path with valid macOS markers
- **THEN** the system returns status `valid`
