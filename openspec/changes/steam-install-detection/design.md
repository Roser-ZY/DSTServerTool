# Design: Steam/DST 安装检测模块

## Context

当前仓库的 Tauri 后端仍是基础启动骨架，尚未形成可复用业务服务层。该变更需要提供稳定的安装检测能力，作为后续服务器配置、模组管理、启动参数生成等功能的前置依赖。

## Design Goals

- 在 Windows 与 macOS 上提供一致的检测语义
- 返回"可解释"的检测结果，而非仅返回布尔安装状态
- 支持自动检测与手动路径输入并行工作
- 将错误码与用户建议绑定，便于前端直接消费

## Non-Goals

- 不处理游戏下载、自动安装、修复安装
- 不处理 Steam 账号、登录状态、运行状态
- 不实现 Linux 检测

## High-Level Flow

```text
detect_installation
  -> detect_steam
  -> resolve_library_folders
  -> detect_dst(app_id=322330)
  -> detect_dst_server(app_id=343050)
  -> aggregate_result_and_errors
```

## Verification Model (Option B)

检测采用分级验证，而不是二元判断。

```rust
pub enum InstallLevel {
    NotFound,
    PathFound,
    ManifestFound,
    ExecutableFound,
}
```

### Level Semantics

- `NotFound`: 未发现目标路径或关键入口文件
- `PathFound`: 目标目录存在，路径结构初步成立
- `ManifestFound`: 发现并成功解析对应 `appmanifest_*.acf`
- `ExecutableFound`: 目标可执行文件存在（判定为"可运行"）

### Component Rules

- Steam:
  - 至少达到 `PathFound` 才认为检测到 Steam 安装
  - 若成功解析 `libraryfolders.vdf`，可提升为更高可信状态（至少达到 `ManifestFound` 语义层）
- DST / DST Dedicated Server:
  - 仅当达到 `ExecutableFound` 才判定为"已安装可用"
  - 若仅达到 `PathFound` 或 `ManifestFound`，返回部分可用信息并附带错误码

## Result Model

保留路径字段，并新增每个组件的层级字段，便于前端按层展示诊断。

```rust
pub struct DetectionResult {
    pub steam_path: Option<String>,
    pub steam_level: InstallLevel,

    pub dst_path: Option<String>,
    pub dst_executable: Option<String>,
    pub dst_level: InstallLevel,

    pub dst_server_path: Option<String>,
    pub dst_server_executable: Option<String>,
    pub dst_server_level: InstallLevel,

    pub errors: Vec<DetectionError>,
}
```

## Error Strategy

- `detect_installation` 采用"尽可能返回信息 + 聚合错误"策略
- 任一子检测失败不应阻断其它组件检测
- 当组件状态低于可运行等级时，必须：
  - 保留当前最高达成层级
  - 写入对应错误码
  - 附带明确建议（安装、修复、手动指定路径）

## Manual Path Strategy

- 手动路径是正式输入，不是异常兜底
- 检测顺序建议：先校验手动路径（若提供），再执行自动检测补全缺失信息
- 手动路径校验与自动检测共用同一套分级规则，保证语义一致

## IPC Contract Notes

- `detect_installation() -> Result<DetectionResult, String>`
  - 建议仅在不可恢复系统级错误时返回 `Err`
  - 普通安装缺失应通过 `DetectionResult.errors` 表达
- `set_manual_path(input: ManualPathInput) -> Result<(), String>`
  - 只负责接收和校验输入
  - 校验失败返回明确错误信息，避免静默失败

## Platform Considerations

- Windows:
  - Steam 根路径优先读取注册表
  - 回退默认路径
- macOS:
  - 以 `~/Library/Application Support/Steam/` 为入口
- 两平台统一使用 `libraryfolders.vdf` + `appmanifest_*.acf` 进行应用定位

## Risks and Mitigations

- `libraryfolders.vdf` / `appmanifest` 格式兼容性风险
  - 通过分级模型降低误判，解析失败时保留已确认层级
- 非标准安装路径风险
  - 通过手动路径输入覆盖默认探测
- 单次失败导致整体验证中断风险
  - 采用聚合错误模型，分组件返回结果

## Open Questions

- `set_manual_path` 是否需要持久化（配置文件或系统存储）
- 分级状态是否需要附带 `source`（manual/auto）字段，便于 UI 展示来源
