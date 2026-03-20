# Capability Spec: Steam Path Search

## Purpose

Define Steam path discovery behavior across supported detection sources, including source attribution, fallback order, and platform-aware root validation checks.

## Requirements

### Requirement: Windows Registry Steam Path Detection

The system SHALL detect Steam installation paths by querying the Windows Registry at `HKEY_LOCAL_MACHINE\SOFTWARE\WOW6432Node\Valve\Steam` and reading the `InstallPath` value.

#### Scenario: Valid registry path found
- **WHEN** Windows Registry contains a valid Steam installation path
- **THEN** system returns the path from registry with source marked as "registry"

#### Scenario: Registry key not found
- **WHEN** Windows Registry key does not exist or is empty
- **THEN** system continues to default path detection without error

#### Scenario: Registry path invalid
- **WHEN** Registry contains a path that does not contain valid Steam structure
- **THEN** system continues to default path detection without error

### Requirement: libraryfolders.vdf Steam Library Detection

The system SHALL parse Steam's `libraryfolders.vdf` file to discover all configured Steam library directories.

#### Scenario: Valid libraryfolders.vdf found
- **WHEN** Steam directory contains valid `libraryfolders.vdf` file
- **THEN** system extracts all `path` entries from libraryfolders sections
- **AND** returns each valid Steam root with source marked as "library_folders"

#### Scenario: No libraryfolders.vdf file
- **WHEN** Steam directory does not contain `libraryfolders.vdf`
- **THEN** system continues without error (single library installation)

#### Scenario: Malformed libraryfolders.vdf
- **WHEN** `libraryfolders.vdf` contains invalid or incomplete data
- **THEN** system ignores malformed entries and processes valid ones
- **OR** continues without error if no valid entries found

### Requirement: Path Detection Source Reporting

The system SHALL report the source of detected Steam paths for debugging and UI display.

#### Scenario: Source attribution for each detection method
- **WHEN** Steam path is successfully detected
- **THEN** result includes `source` field with one of: "manual", "registry", "library_folders", "windows-default", "macos-default"

### Requirement: Detection Order and Priority

The system SHALL attempt path detection in the following order:
1. Manual path provided by user (highest priority)
2. Windows Registry (Windows only)
3. Default installation directories
4. libraryfolders.vdf parsed from any discovered Steam directory

#### Scenario: Manual path takes precedence
- **WHEN** user provides manual path AND system detects paths via registry/defaults
- **THEN** system validates manual path and returns it if valid

#### Scenario: Fallback to secondary sources
- **WHEN** primary detection method fails or returns invalid path
- **THEN** system falls back to next detection method in priority order

### Requirement: Steam Root Validation

The system SHALL validate detected Steam paths using platform-aware Steam root rules.

#### Scenario: Valid Steam root
- **WHEN** path satisfies required Steam markers for the current platform
- **THEN** path is marked as status "valid"

#### Scenario: Invalid Steam root
- **WHEN** path is missing required platform markers or directories
- **THEN** path is marked as status "invalid" with validationErrors listing missing paths

#### Scenario: macOS layout validation
- **WHEN** validation runs on macOS
- **THEN** system checks macOS Steam markers (e.g. `registry.vdf`, `Steam.AppBundle`, `config/libraryfolders.vdf`)
- **AND** does not require Windows-specific `steamapps/common` + `steamapps/workshop` combination as mandatory root criteria
