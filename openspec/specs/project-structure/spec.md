# Capability Spec: 项目目录结构

## Purpose

定义 DSTServerTool 的目录结构基线，保证前端、Tauri 后端、规格文档与项目文档的职责分区清晰。实现细节允许演进，但整体结构应保持大致一致。

## Background

技术栈：
- 桌面容器：Tauri 2.x
- 前端：Vue 3 + TypeScript + Vite + Reka UI + Tailwind CSS
- 后端：Rust（Tauri Command）+ Python 3.x 兼容层

当前实现基线（截至 2026-03-16）：
- Tauri 桌面应用壳已存在
- 前端已迁移到 Vue 3 + Vite + shadcn-vue，并已有可折叠基础壳层
- Python 模组更新核心已可执行

## Requirements

### Requirement: Repository Top-Level Structure Baseline

系统必须提供稳定的仓库顶层结构，以支持桌面端开发、规格管理和文档沉淀。该结构可以扩展，但核心职责分区应保持一致。

#### Scenario: Baseline directories are present

- **WHEN** 开发者初始化或维护仓库结构
- **THEN** 仓库包含与职责对应的核心目录（如前端源码、Tauri 后端、OpenSpec 规格目录和项目文档目录）
- **AND** 目录命名或组织细节可调整，但职责边界不应混淆

#### Scenario: Structure allows controlled variation

- **WHEN** 新功能引入额外模块或工具目录
- **THEN** 允许新增目录或子层级
- **AND** 不要求与基线逐字符一致
- **AND** 新增结构应与既有职责分区保持大致一致

### Requirement: Frontend and Tauri Backend Separation

系统必须将前端应用与 Tauri 后端保持清晰分层，前端代码位于前端源码目录，后端命令与业务逻辑位于 `src-tauri` 范围内。

#### Scenario: Frontend and backend responsibilities are separated

- **WHEN** 开发者添加新功能
- **THEN** UI 路由、页面和组件位于前端目录
- **AND** 本地能力、系统调用、路径探测等后端逻辑位于 Tauri 后端目录

### Requirement: Tauri Backend Internal Module Convention

Tauri 后端应采用命令层、服务层、工具层的模块约定，以便将 IPC 入口、业务逻辑和通用能力解耦。

#### Scenario: New backend feature follows module convention

- **WHEN** 新增后端能力（例如安装检测）
- **THEN** IPC 命令放在命令层
- **AND** 业务编排放在服务层
- **AND** 错误码、通用模型或工具放在工具层或等价层级

### Requirement: Project Directory Tree

系统必须将目录结构按职责分区组织，目录命名与组织细节可演进，但各分区的职责边界应保持清晰。

#### Scenario: Frontend source tree

- **WHEN** 前端代码位于 `src/`
- **THEN** 包含 `components/ui/`（Reka UI 组件库）、`lib/`（工具与辅助函数）、`router/`（路由配置）、`views/`（页面组件）

#### Scenario: Tauri backend tree

- **WHEN** Tauri 后端代码位于 `src-tauri/`
- **THEN** 包含 `src/main.rs`（入口点）、`Cargo.toml`（依赖配置）、`tauri.conf.json`（Tauri 配置）
- **AND** `src/` 下按模块约定组织子目录（如 `commands/`、`services/`、`utils/`）

#### Scenario: Specification and documentation tree

- **WHEN** 规格文档与项目文档位于仓库内
- **THEN** OpenSpec 规格目录位于 `openspec/`，项目文档位于 `docs/`
- **AND** `openspec/changes/` 用于跟踪变更，`openspec/specs/` 用于主规格
- **AND** `docs/<topic>/` 用于按功能组织的项目文档

#### Scenario: Build output separation

- **WHEN** 构建产物生成
- **THEN** 前端构建产物位于 `dist/`
- **AND** 构建产物不混入源码目录
