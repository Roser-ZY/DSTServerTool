# Feature Specification: DST-1 模组更新检测与替换

**Feature Branch**: `001-dst-mod-update`  
**Created**: 2026-02-07  
**Status**: Draft  
**Input**: User description: "DST-1 模组更新检测与替换 检测 Windows 下的 steam 安装目录下的 `steamapps/workshop` 和 `steamapps/common` 下的饥荒联机版和饥荒联机版服务器目录中的饥荒模组，检查模组的更新日期，将更新的模组目录拷贝到饥荒联机版服务器目录下的 mods 目录中。 需要一个 mods_update.py 脚本，在 Windows 环境下运行。 1. 支持指定选择 steam 目录。 2. 在 `steamapps/common` 目录下，寻找 Dont Starve Together Dedicated Server 目录和 Dont Starve Together 目录。 3. 在 `Dont Starve Together Dedicated Server/mods` 目录下，遍历检查当前仓库的 `workshop-`前缀的目录及其编号，并记录。 4. 在 `steamapps/workshop` 目录下，以及 `Dont Starve Together/mods` 目录下，搜索所有与第 3 部匹配的编号目录，并将其添加 `workshop-` 前缀后，拷贝并替换到"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - 一键更新服务器模组 (Priority: P1)

服务器管理员需要在 Windows 环境下运行更新流程，自动检查并把更新的模组同步到饥荒联机版服务器的 `mods` 目录。

**Why this priority**: 这是核心价值，直接节省手工比对与拷贝的时间。

**Independent Test**: 使用一个包含多个 `workshop-` 目录的服务器 `mods` 目录运行更新流程，应只更新较新的模组并保留未更新的模组不变。

**Acceptance Scenarios**:

1. **Given** 服务器 `mods` 目录包含若干 `workshop-<id>` 目录，且来源目录存在对应模组，**When** 运行更新流程，**Then** 仅当来源模组更新时间更晚时才替换服务器目录。
2. **Given** 来源目录未找到与服务器 `workshop-<id>` 对应的模组，**When** 运行更新流程，**Then** 该模组不会被删除或替换，并在结果中被标记为未找到。

---

### User Story 2 - 指定 Steam 安装目录 (Priority: P2)

管理员可以指定 Steam 安装目录，以适配非默认安装路径或多库目录的场景。

**Why this priority**: 不同机器安装路径差异很大，必须支持手动指定以避免找不到模组。

**Independent Test**: 使用一个非默认路径的 Steam 安装目录，运行更新流程并验证能成功定位到必要目录。

**Acceptance Scenarios**:

1. **Given** 用户提供有效的 Steam 安装目录路径，**When** 运行更新流程，**Then** 系统在该路径下正确定位 `steamapps/common` 和 `steamapps/workshop`。

---

### User Story 3 - 更新结果可追溯 (Priority: P3)

管理员需要清楚知道哪些模组被更新、哪些未更新以及原因。

**Why this priority**: 便于问题排查与确认更新结果。

**Independent Test**: 运行更新流程后，查看输出或日志即可判定每个模组的处理结果。

**Acceptance Scenarios**:

1. **Given** 更新流程完成，**When** 用户查看结果，**Then** 可以看到每个模组的状态（已更新、未更新、未找到、失败）及对应原因。

---

### Edge Cases

- 当指定的 Steam 目录不存在或缺少 `steamapps` 结构时如何提示？
- 当服务器 `mods` 目录为空或不存在 `workshop-` 前缀目录时如何处理？
- 当来源模组存在但无法读取或拷贝失败时如何处理并记录？
- 当来源模组同时在 `steamapps/workshop` 和 `Dont Starve Together/mods` 中存在时，如何选择更新来源？

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: 系统 MUST 支持用户指定 Steam 安装目录作为扫描起点。
- **FR-002**: 系统 MUST 在指定 Steam 目录下定位 `steamapps/common` 和 `steamapps/workshop`。
- **FR-003**: 系统 MUST 在 `steamapps/common` 内找到 `Dont Starve Together Dedicated Server` 与 `Dont Starve Together` 目录。
- **FR-004**: 系统 MUST 扫描服务器 `Dont Starve Together Dedicated Server/mods` 中所有 `workshop-<id>` 目录并记录其编号。
- **FR-005**: 系统 MUST 在 `steamapps/workshop` 与 `Dont Starve Together/mods` 中查找与记录编号匹配的模组目录。
- **FR-006**: 系统 MUST 基于模组更新时间判断是否需要替换服务器 `mods` 目录中的对应模组。
- **FR-007**: 系统 MUST 在需要更新时将来源模组复制并替换到服务器 `mods` 目录中的同名 `workshop-<id>` 目录。
- **FR-008**: 系统 MUST 在找不到来源模组时跳过替换并记录为未找到。
- **FR-009**: 系统 MUST 输出每个模组的处理结果与原因，覆盖已更新、未更新、未找到、失败四种状态。
- **FR-010**: 系统 MUST 在 Windows 环境下可运行。
- **FR-011**: 系统 MUST 提供名为 `mods_update.py` 的可执行更新脚本入口，允许通过命令行传入路径参数来触发更新流程。
- **FR-012**: 系统 MUST 提供可被其他项目调用的核心更新能力，允许通过传入路径参数复用该功能。

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

### Key Entities *(include if feature involves data)*

- **Steam 安装目录**: 用户指定的 Steam 根路径，用于定位 `steamapps` 结构。
- **服务器模组目录**: `Dont Starve Together Dedicated Server/mods` 中的 `workshop-<id>` 目录集合。
- **来源模组目录**: `steamapps/workshop` 和 `Dont Starve Together/mods` 中与 `workshop-<id>` 匹配的模组目录。
- **模组记录**: 每个 `workshop-<id>` 的编号、来源位置、更新时间与处理结果。

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 在包含 200 个模组的场景下，完成一次完整扫描与更新流程的时间不超过 5 分钟。
- **SC-002**: 对于有更新来源的模组，100% 被更新到服务器 `mods` 目录中。
- **SC-003**: 更新流程完成后，100% 的模组都有明确的处理结果与原因输出。
- **SC-004**: 管理员在单次运行中完成更新，无需手工复制或手工比对模组文件。

## Assumptions

- 默认以模组目录的“最后修改时间”作为更新时间判断依据。
- 当来源模组同时存在于两个来源路径时，选择更新时间更晚的来源进行替换。
- 更新流程仅影响服务器 `mods` 目录中的 `workshop-<id>` 目录，不新增不在列表中的模组。

## Dependencies

- 目标机器已安装 Steam 且包含 `steamapps` 目录结构。
- 已安装并包含 `Dont Starve Together` 与 `Dont Starve Together Dedicated Server` 目录。
