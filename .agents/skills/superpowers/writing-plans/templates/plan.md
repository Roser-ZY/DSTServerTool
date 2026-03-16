# [Feature Name] Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use executing-plans to implement this plan task-by-task.

**Goal:** [一句话描述本计划要构建的功能]

**Architecture:** [2-3 句描述整体架构方案。包括核心服务/模块划分、层级关系（Common/Client/Server）、关键设计决策]

**Tech Stack:** [关键技术栈和依赖，如 Lua (自研引擎 API)、ServiceLocator、特定框架等]

**FR Documents:**
- `docs/<feature>/FR-0001_<name>/FR-0001_<name>.md`
- `docs/<feature>/FR-0002_<name>/FR-0002_<name>.md`
- [... 列出所有关联的 FR 文档路径]

**Design Document:** `docs/plans/YYYY-MM-DD-<feature-name>-design.md` _(如有)_

---

## Task Overview

| Task | Component | FR | Dependencies |
|------|-----------|-----|--------------|
| 1 | [组件名称] | FR-0001 | 无 |
| 2 | [组件名称] | FR-0002 | Task 1 |
| 3 | [组件名称] | FR-0003 | Task 2 |
| N | [组件名称] | FR-000N | Task X |

> **注意**：[描述任务间的依赖关系和执行顺序约束。例如：哪些 Task 可以并行，哪些必须串行]

---

## Task 1: [组件名称]

**FR:** FR-0001

**Files:**
- Create: `exact/path/to/new_file.lua`
- Modify: `exact/path/to/existing_file.lua`

### Step 1: [具体操作描述]

[操作说明文字]

```lua
-- exact/path/to/new_file.lua

-- 完整的代码实现，不允许 "添加验证逻辑" 等模糊描述
-- 必须包含所有 require、常量定义、类型注解
local SomeModule = require("full.require.path")

---@class NewClass
---@field _field_name type 字段描述
local NewClass = {}
NewClass.__index = NewClass

function NewClass.new()
    local self = setmetatable({}, NewClass)
    self._field_name = initial_value
    return self
end

return NewClass
```

### Step 2: [具体操作描述]

[操作说明文字。如果是修改现有文件，必须说明：]
- **修改位置**：在 `[函数名/行号范围]` 之后/之前
- **修改内容**：添加/替换/删除什么代码

```lua
-- 完整的修改代码
```

### Step 3: Verify

[具体验证步骤，例如：]
- 确认引擎启动无报错
- 确认 `SL:get("service_name")` 可正常获取服务实例
- 确认日志输出 `[ModuleName] Initialized`

### Step 4: Commit

```bash
git add path/to/file1.lua
git add path/to/file2.lua
git commit -m "feat(<scope>): <description> (FR-000N)"
```

---

## Task 2: [组件名称]

**FR:** FR-0002

**Files:**
- Modify: `exact/path/to/file.lua`

### Step 1: [具体操作描述]

[... 同 Task 1 结构，每个 Step 都包含完整代码 ...]

### Step N: Verify

[验证步骤]

### Step N+1: Commit

```bash
git add path/to/files
git commit -m "feat(<scope>): <description> (FR-000N)"
```

---

<!--
  根据实际需求重复 Task 结构。
  每个 Task 必须包含：
  1. FR 引用
  2. 精确的文件路径列表（Create / Modify）
  3. 分步骤实现，每步包含完整可执行代码
  4. Verify 步骤（具体的验证方法和预期结果）
  5. Commit 步骤（精确的 git 命令和 conventional commit message）
-->

---

## 最终验证清单

| # | 检查项 | 验证方式 |
|---|--------|----------|
| 1 | [功能点 1] | [具体验证方法] |
| 2 | [功能点 2] | [具体验证方法] |
| 3 | [功能点 N] | [具体验证方法] |
| N | 所有 Log 输出正常，无报错 | 引擎控制台 / 日志文件检查 |

---

## 代码实现验证

> **⚠️ 以下两个技能的使用是强制性的（MANDATORY），不可跳过。**

### 1. Subagent-Driven Development（必须）

使用 `subagent-driven-development` SKILL 执行本计划的所有 Task：

- **每个 Task 分派独立的 Implementer Subagent** — 保证上下文隔离，避免跨任务污染
- **每个 Task 完成后执行三阶段审查**：
  1. **Spec Compliance Review** — 确认实现与 FR 文档功能逻辑一致（代码可以不同，但输出和结果必须与 FR 一致）
  2. **Code Functional Implementation Review** — 确认代码功能逻辑与 Plan 和 FR 文档描述一致
  3. **Code Quality Review** — 确认代码质量符合项目规范（CODE_STYLE.md）
- **所有 Task 完成后执行最终全量代码审查**
- **审查未通过必须修复后重新审查**，不允许跳过

### 2. Test-Driven Development（必须）

使用 `test-driven-development` SKILL 指导每个 Task 的实现过程：

- **先写验证逻辑，再写实现代码** — 即使项目不使用测试框架，也必须定义清晰的验证标准
- **每个 Step 必须有可观测的预期结果** — 如日志输出、API 返回值、引擎行为
- **实现完成后立即执行验证** — 确认行为符合预期后才进入下一步
- **验证失败必须修复后重新验证**，不允许跳过

### 执行流程

```
Plan Ready
  ↓
[For each Task]
  ├── Dispatch Implementer Subagent (subagent-driven-development)
  │     └── Implementer follows TDD (test-driven-development):
  │           ├── Define expected behavior / verification criteria
  │           ├── Implement code
  │           ├── Verify behavior matches expectation
  │           └── Commit
  ├── Dispatch Spec Reviewer → Pass? → If No → Fix → Re-review
  ├── Dispatch Code Functional Reviewer → Pass? → If No → Fix → Re-review
  └── Dispatch Code Quality Reviewer → Pass? → If No → Fix → Re-review
  ↓
[All Tasks Complete]
  ├── Final Code Review (entire implementation)
  └── Use finishing-a-development-branch SKILL
```

### 补充技能（推荐但非强制）

- 使用 `verification-before-completion` SKILL — 在声明任务完成前运行验证命令，确保证据先于断言
- 使用 `requesting-code-review` SKILL — 为 Reviewer Subagent 提供标准化的审查模板
- 使用 `git-commit` SKILL — 生成符合 Conventional Commit 规范的提交信息
