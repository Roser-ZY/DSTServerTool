## Why

用户将 Steam 自定义安装在非默认路径（如 D:\Steam），导致现有路径检测功能无法找到 Steam 安装目录。这影响了模组更新等核心功能的使用。

当前 Steam 路径检测仅依赖默认安装目录探测，无法处理用户将 Steam 安装在其他位置或配置了多个 Steam 库目录的情况。

此外，在 macOS 上当 Steam 未安装时，路径检测可能错误地返回无关目录（如 `~/Documents/New Project`）作为 Steam 根目录，导致误判和后续工作流失败。

## What Changes

- 增强 Windows 平台的路径检测，添加注册表查询能力
- 新增 Steam `libraryfolders.vdf` 文件解析，检测所有配置的 Steam 库目录
- 统一路径探测来源报告，支持 UI 显示路径来源
- 增强 Steam 根目录验证，使用 Steam 专属文件标记（如 `config/libraryfolders.vdf`）防止误报
- macOS 自动探测时，在未找到有效 Steam 根目录时应返回 `not_found`，而非返回无效猜测路径
- 按 macOS 真实目录布局修正验证规则（以 `registry.vdf`、`Steam.AppBundle`、`config/` 等 Steam 标记为准，而不是复用 Windows 目录结构）
- 新增 `detect_steam_path` Tauri 命令，提供统一的后端路径检测入口
- 在前端 UI 中集成路径显示、手动覆盖和错误提示

## Capabilities

### New Capabilities

- `steam-path-detection`: 在桌面应用中自动检测 Steam 安装路径，支持 macOS 与 Windows
- `steam-path-search`: 完善 Steam 路径搜索服务，支持以下检测手段：
  - 默认安装目录探测（现有功能）
  - Windows 注册表 `HKEY_LOCAL_MACHINE\SOFTWARE\WOW6432Node\Valve\Steam` 中的 InstallPath
  - Steam 安装目录下 `steamapps/libraryfolders.vdf` 配置的所有库目录
  - macOS 常见 Steam 路径
- `steam-root-validation`: 验证检测到的 Steam 根目录是否包含 Steam 专属文件标记，防止误报
  - 采用平台感知（platform-aware）的验证规则，避免 macOS 套用 Windows `steamapps/common` 与 `steamapps/workshop` 目录校验
- `prototype-1-test-harness`: 为 prototype-1 提供独立、后端专用、单入口的跨平台自动化闭环测试（覆盖注册表、默认目录、libraryfolders.vdf 三类检测路径）
  - 包含 `tests/prototype-1/mock` 虚拟 Steam 目录集（Windows/macOS）用于无真实安装环境下的回归测试
  - 当系统已检测到真实 Steam 安装时，mock 路径不参与"找到路径"类测试

### Modified Capabilities

- `001-dst-mod-update`: 使用检测到的 Steam 根作为模组更新操作的默认源路径

## Impact

- **Modified Code**:
  - `src-tauri/src/services/path/path_detection.rs` - 核心路径检测逻辑
  - `src/lib/steam-path.ts` - 前端路径相关工具函数
- **New Files**: `src-tauri/src/services/path/library_folders.rs` - VDF 解析模块
- **New Files**: `tests/prototype-1/` 下的跨平台测试入口及测试数据
- **New Files**: `tests/prototype-1/mock/` 下按平台组织的虚拟 Steam 目录 fixture
- **Dependencies**: 使用 Rust 内置方式读取 Windows 注册表
- **Compatibility**: 需兼容 Windows 现有路径探测行为，同时修正 macOS 路径布局识别
- **Test Scope**: 闭环测试仅覆盖后端路径检测能力，不包含任何前端测试步骤
