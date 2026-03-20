## MODIFIED Requirements

### Requirement: Steam Path Input for Mod Update
The mod update flow SHALL use a validated Steam path as its primary source configuration. The path MAY come from automatic Steam path detection or manual user input, but MUST be validated before update execution.

#### Scenario: Use auto-detected valid path by default
- **WHEN** Steam path detection returns status `valid`
- **THEN** the mod update flow pre-fills and uses that path as the default source path

#### Scenario: Fallback to manual path
- **WHEN** detection status is `not_found` or `invalid`
- **THEN** the user MUST provide a manual Steam path that passes validation before the update starts

#### Scenario: Reject unvalidated path
- **WHEN** the user attempts to run mod update with a path that has not passed validation
- **THEN** the system blocks execution and displays the validation reason
