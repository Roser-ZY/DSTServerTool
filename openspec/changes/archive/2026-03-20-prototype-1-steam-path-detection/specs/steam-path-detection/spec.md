## ADDED Requirements

### Requirement: Detect Steam Path Across Supported Platforms
The system SHALL detect Steam installation path candidates on application startup or when the user triggers path detection, supporting Windows and macOS probe rules.

#### Scenario: Detection finds valid path
- **WHEN** the user opens the Steam path section and a default Steam installation is present
- **THEN** the system returns status `valid` with the detected absolute path and detection source

#### Scenario: Detection finds no candidate
- **WHEN** the detection routine checks supported probe locations and none match
- **THEN** the system returns status `not_found` without crashing and with an actionable message

### Requirement: Validate Detected or Manual Steam Path
The system SHALL validate detected and user-provided paths against expected Steam structure required by DST workflows, including `steamapps/common` and `steamapps/workshop`.

#### Scenario: Manual override is valid
- **WHEN** the user enters a path such as `D:\\Steam` and triggers validation
- **THEN** the system marks it as `valid` and allows DST mod update workflows to use it

#### Scenario: Path is invalid
- **WHEN** the path does not contain required subpaths
- **THEN** the system returns status `invalid` and validation errors describing missing directories

### Requirement: Preserve Manual Fallback and Retry
The system MUST allow users to override any detected path, retry detection, and revalidate before persisting or using the path in DST operations.

#### Scenario: User overrides incorrect auto-detection
- **WHEN** an auto-detected path is invalid for the local setup
- **THEN** the user can replace it with a manual path and proceed after successful validation
