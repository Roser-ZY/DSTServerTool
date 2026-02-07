---

description: "Task list for DST-1 模组更新检测与替换"
---

# Tasks: DST-1 模组更新检测与替换

**Input**: Design documents from `/specs/001-dst-mod-update/`
**Prerequisites**: plan.md (required), spec.md (required), research.md, data-model.md, contracts/

**Tests**: Not requested in the specification; no test tasks included.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [x] T001 Create core module package structure in mods_update/__init__.py and mods_update/core.py
- [x] T002 Create CLI entry script scaffold in mods_update.py
- [x] T003 [P] Add minimal README usage note in /Users/roserhan/Desktop/Projects/DSTServerTool/README.md referencing specs/001-dst-mod-update/quickstart.md

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T004 Implement path validation and Steam directory discovery helpers in mods_update/core.py
- [x] T005 Implement workshop id parsing and server mods scan in mods_update/core.py
- [x] T006 Implement source mods discovery across steamapps/workshop and Dont Starve Together/mods in mods_update/core.py
- [x] T007 Implement update decision logic (mtime comparison and preferred source selection) in mods_update/core.py
- [x] T008 Implement safe copy/replace operation with failure capture in mods_update/core.py
- [x] T009 Define UpdateResult structure and status normalization in mods_update/core.py

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - 一键更新服务器模组 (Priority: P1) 🎯 MVP

**Goal**: 自动扫描服务器模组并更新替换到最新版本

**Independent Test**: 使用包含多个 workshop-<id> 的服务器 mods 目录运行流程，仅更新时间更晚的模组被替换，其余保持不变

### Implementation for User Story 1

- [x] T010 [US1] Implement end-to-end update pipeline orchestration in mods_update/core.py
- [x] T011 [US1] Wire core update pipeline to return UpdateResult list in mods_update/core.py
- [x] T012 [US1] Ensure only existing server workshop-<id> directories are processed in mods_update/core.py

**Checkpoint**: User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - 指定 Steam 安装目录 (Priority: P2)

**Goal**: 支持通过命令行指定 Steam 安装目录并驱动更新流程

**Independent Test**: 提供非默认 Steam 路径运行 CLI，能够正确定位 steamapps 结构并执行更新流程

### Implementation for User Story 2

- [x] T013 [US2] Parse and validate --steam-path argument in mods_update.py
- [x] T014 [US2] Call core update interface with provided path in mods_update.py
- [x] T015 [US2] Provide clear CLI error messages for invalid/missing Steam paths in mods_update.py

**Checkpoint**: User Story 2 should be independently functional and testable

---

## Phase 5: User Story 3 - 更新结果可追溯 (Priority: P3)

**Goal**: 输出每个模组的处理状态与原因

**Independent Test**: 运行更新流程后输出包含每个模组的状态与原因

### Implementation for User Story 3

- [x] T016 [US3] Format UpdateResult entries for human-readable output in mods_update.py
- [x] T017 [US3] Emit status summary counts (updated/unchanged/missing/failed) in mods_update.py
- [x] T018 [US3] Ensure failure paths capture actionable reasons in mods_update/core.py

**Checkpoint**: All user stories should now be independently functional

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [x] T019 [P] Align quickstart examples with final CLI behavior in /Users/roserhan/Desktop/Projects/DSTServerTool/specs/001-dst-mod-update/quickstart.md
- [x] T020 Add minimal inline usage help in mods_update.py
- [x] T021 Run quickstart validation manually and note any caveats in /Users/roserhan/Desktop/Projects/DSTServerTool/specs/001-dst-mod-update/quickstart.md

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P3)
- **Polish (Final Phase)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - Independently testable with CLI
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) - Depends on UpdateResult output from US1

### Parallel Opportunities

- T003 can run in parallel with T001-T002
- T004-T009 are sequential within core.py, but can be split by function if staffed
- T013-T017 can be parallelized with US1 completion if UpdateResult interface is stable

---

## Parallel Example: User Story 1

```bash
Task: "Implement end-to-end update pipeline orchestration in mods_update/core.py"
Task: "Ensure only existing server workshop-<id> directories are processed in mods_update/core.py"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1
4. STOP and validate User Story 1 independently

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Validate independently (MVP)
3. Add User Story 2 → Validate independently
4. Add User Story 3 → Validate independently

---

## Validation

- All tasks use required checklist format with IDs, optional [P], and [US#] labels for story tasks
- Each task includes an explicit file path
