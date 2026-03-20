# Capability Spec: Prototype-1 Test Harness

## Purpose

Define the backend-only automated test harness for prototype-1 Steam path detection, including fixture layout, documentation, and cross-platform execution flow.

## Requirements

### Requirement: Prototype-1 test artifacts MUST live under tests/prototype-1
The system MUST place prototype-1 closed-loop automated tests under `tests/prototype-1` and include a `README.md` describing current automated coverage and execution flow.

#### Scenario: Test documentation exists
- **WHEN** prototype-1 automated test assets are generated
- **THEN** `tests/prototype-1/README.md` exists
- **AND** README documents current test items, execution steps, and platform-specific branches

### Requirement: Single-entry cross-platform test flow
The system MUST provide one unified test entrypoint for prototype-1, and the entrypoint MUST dispatch platform-specific test branches internally.

#### Scenario: Unified entrypoint on Windows
- **WHEN** user executes the single prototype-1 test entrypoint on Windows
- **THEN** tests include registry detection, default install directory detection, and `libraryfolders.vdf` detection flows

#### Scenario: Unified entrypoint on macOS
- **WHEN** user executes the single prototype-1 test entrypoint on macOS
- **THEN** tests run platform-relevant default path and `libraryfolders.vdf` detection flows
- **AND** test routing is chosen by runtime platform detection

### Requirement: Closed-loop test scope isolation
Prototype-1 automated tests MUST be backend-only, independently executable, and scoped strictly to prototype-1 path detection behavior.

#### Scenario: Test scope excludes frontend
- **WHEN** prototype-1 automated tests execute
- **THEN** no frontend component rendering or UI test steps are required

#### Scenario: Independent execution
- **WHEN** user runs prototype-1 test entrypoint in isolation
- **THEN** tests execute without requiring unrelated business tasks or modules
- **AND** only necessary test framework/third-party dependencies are allowed

### Requirement: Mock fixtures for cross-platform virtual Steam layouts
The system MUST provide a dedicated `tests/prototype-1/mock` directory containing platform-specific virtual Steam directory fixtures.

#### Scenario: Mock directory structure exists
- **WHEN** prototype-1 test harness is initialized
- **THEN** `tests/prototype-1/mock` exists
- **AND** it contains platform fixture sets for Windows and macOS Steam layouts

#### Scenario: Mock fixtures are used only when real install is not found
- **WHEN** running "found-path" test flows
- **THEN** harness MUST prefer real detected Steam installation
- **AND** harness MUST fallback to `tests/prototype-1/mock` only when the current system cannot find a real Steam install
