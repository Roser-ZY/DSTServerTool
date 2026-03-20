# Prototype-1 Closed-Loop Tests

This directory contains backend-only automated tests for `prototype-1` Steam path detection.

## Scope

- Windows registry detection flow
- Platform default install directory detection flow
- `libraryfolders.vdf`-based detection flow
- macOS real-layout validation (`registry.vdf`, `Steam.AppBundle`, `config/libraryfolders.vdf`)

These tests do not render frontend components and do not depend on non-Prototype-1 business tasks.

## Entry Point

Run the unified entrypoint:

```bash
npm run test:prototype-1
```

The entrypoint detects the current runtime platform and dispatches the matching backend test branch.

## Real Install vs Mock Fixtures

- If the current machine has a real Steam install for the active platform, the harness prefers a real-install smoke test.
- If no real install is detected, the harness falls back to fixture directories under `tests/prototype-1/mock/` for "found-path" scenarios.
- Mock fixtures exist for Windows and macOS.

## Mock Fixture Layout

- `mock/windows/`
  - `default-root/`
  - `registry-root/`
  - `library-host/`
  - `library-root/`
- `mock/macos/`
  - `valid-root/`
  - `invalid-root/`
  - `library-host/`
  - `library-root/`

Some fixture files use placeholders and are copied to a temp directory before execution so absolute paths can be injected into `libraryfolders.vdf`.

Directory placeholders (`.gitkeep`) are committed for empty folders required by validation rules (for example `steamapps/common`, `steamapps/workshop`, and `Steam.AppBundle/Steam`) so fixture structure stays stable across platforms.

## Current Automated Coverage

- Real-install smoke test for the current platform when a real install is present
- Mock default-path detection for the current platform when no real install is present
- Mock `libraryfolders.vdf` detection for the current platform when no real install is present
- Mock Windows registry/default/libraryfolders flows on Windows
- Manual invalid-path validation checks for macOS real-layout markers
