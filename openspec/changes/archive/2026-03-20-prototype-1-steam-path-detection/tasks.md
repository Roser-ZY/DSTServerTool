## 1. [Rust] Steam path detection backend

- [x] 1.1 [Rust] Add `detect_steam_path` Tauri command and register it in the command handler.
- [x] 1.2 [Rust] Implement OS-specific probe candidates for Windows and macOS Steam default locations.
- [x] 1.3 [Rust] Implement path validation helper for required DST-related Steam structure (`steamapps/common`, `steamapps/workshop`).
- [x] 1.4 [Rust] Return structured IPC payload (`status`, `path`, `source`, `validationErrors`) with deterministic error mapping.

## 2. [Vue] Frontend IPC and state integration

- [x] 2.1 [Vue] Add TypeScript types and IPC wrapper for `detect_steam_path` response handling.
- [x] 2.2 [Vue] Build `SteamPathCard` and `SteamPathInput` UI sections in the existing setup/mod update view.
- [x] 2.3 [Vue] Implement UX states for `valid`, `invalid`, and `not_found`, including retry and manual override actions.
- [x] 2.4 [Vue] Persist selected validated path and prevent proceeding when path is not validated.

## 3. Mod update flow alignment

- [x] 3.1 Update `001-dst-mod-update` flow to prefill from auto-detected path when status is `valid`.
- [x] 3.2 Enforce fallback behavior requiring a manually validated path when detection is unavailable or invalid.
- [x] 3.3 Block update execution on unvalidated paths and surface validation reason to users.

## 4. Verification

- [x] 4.1 Verify detection and validation behavior with representative paths on at least one desktop platform.
- [x] 4.2 Run frontend type check/build (`vue-tsc --noEmit`, `npm run build`) and fix any regressions.
- [x] 4.3 Run Tauri compile verification (`npm run tauri build` or targeted Cargo build) for command integration.

## 5. Setup

- [x] 5.1 Create `src-tauri/src/services/path/library_folders.rs` module file
- [x] 5.2 Add `library_folders` module to `src-tauri/src/services/path/mod.rs`

## 6. libraryfolders.vdf Parser Implementation [Rust]

- [x] 6.1 Implement `parse_library_folders` function to read and parse VDF file
- [x] 6.2 Extract `path` entries from libraryfolders sections
- [x] 6.3 Handle malformed VDF entries gracefully (skip invalid, continue parsing)
- [x] 6.4 Return `Vec<PathBuf>` of discovered Steam library paths

## 7. Windows Registry Query [Rust]

- [x] 7.1 Add Windows-specific registry reading in `path_detection.rs`
- [x] 7.2 Query `HKEY_LOCAL_MACHINE\SOFTWARE\WOW6432Node\Valve\Steam\InstallPath`
- [x] 7.3 Implement fallback behavior when registry read fails
- [x] 7.4 Wrap in `#[cfg(target_os = "windows")]` conditional compilation

## 8. Integrate Detection Logic [Rust]

- [x] 8.1 Modify `detect_steam_path_internal` to implement detection order:
  - Manual path → Registry → Default paths → libraryfolders.vdf
- [x] 8.2 Set `source` field appropriately ("registry", "library_folders", etc.)
- [x] 8.3 Ensure first valid path is returned (no need to find all)

## 9. Testing

- [x] 9.1 Add unit tests for `library_folders.rs` with mock VDF content
- [x] 9.2 Run existing tests in `src-tauri/tests/commands_test.rs`
- [ ] 9.3 Test with actual D:\Steam installation (user's environment)

## 10. Steam Root Validation [Rust]

- [x] 10.1 Add a helper to validate Steam-specific root markers (e.g. `config/libraryfolders.vdf`) in addition to required directories
- [x] 10.2 Update automatic probe handling so invalid auto-detected candidates continue probing and fall back to `not_found`
- [x] 10.3 Ensure manual path validation returns explicit `validationErrors` listing missing Steam markers or directories

## 11. macOS False-Positive Tests [Rust]

- [x] 11.1 Add regression test: non-Steam folder resembling partial Steam layout must not be returned
- [x] 11.2 Add regression test: absent Steam returns `not_found` instead of surfacing unrelated folder

## 12. Final Verification

- [x] 12.1 Run `cargo test` in `src-tauri`
- [x] 12.2 Run `npm run tauri build`
- [x] 12.3 Verify on macOS (no real Steam) that detection returns `not_found`

## 13. macOS Real Layout Correction [Rust]

- [x] 13.1 Refactor root validation to use platform-aware marker sets (Windows vs macOS)
- [x] 13.2 Implement macOS marker rule using real Steam layout (`registry.vdf`, `Steam.AppBundle`, `Steam.AppBundle/Steam`, `config/libraryfolders.vdf`)
- [x] 13.3 Add regression test: macOS real Steam root markers should return `valid`
- [x] 13.4 Add regression test: macOS path missing real markers should return `invalid` for manual input
- [x] 13.5 Re-run `cargo test` and `npm run tauri build` after macOS layout correction

## 14. Prototype-1 Closed-Loop Test Harness [Rust]

- [x] 14.1 Create `tests/prototype-1/README.md` and document current automated test coverage and execution steps
- [x] 14.2 Create a single test entrypoint under `tests/prototype-1/` that dispatches platform-specific test flows
- [x] 14.3 Implement Windows test branch: registry detection, default install directory detection, `libraryfolders.vdf` detection
- [x] 14.4 Implement macOS test branch: platform default install directory detection and `libraryfolders.vdf` detection
- [x] 14.5 Ensure tests are backend-only and do not include any frontend testing steps
- [x] 14.6 Ensure test harness runs independently and only depends on Prototype-1 plus required test libraries
- [x] 14.7 Run the unified test entrypoint and verify platform-based routing works as expected

## 15. Mock Fixtures [Rust]

- [x] 15.1 Create `tests/prototype-1/mock/` with platform fixture directories for Windows/macOS virtual Steam layouts
- [x] 15.2 Update `tests/prototype-1/README.md` with mock fixture policy and usage rules
- [x] 15.3 Implement fallback rule in test harness: use mock "found-path" fixtures only when real install is not detected
- [x] 15.4 Re-run unified test entrypoint and confirm backend-only scope with no frontend dependency
