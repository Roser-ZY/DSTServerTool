# Steam/DST 安装检测模块设计

## 概述

在 `src-tauri/services/` 目录下新增业务逻辑模块，用于检测 Steam、Don't Starve Together (DST) 客户端和 DST Dedicated Server 的安装状态和目录。

## 功能需求

### F1: Steam 安装检测

- 检测 Steam 是否已安装
- 获取 Steam 安装目录
- Windows: 优先读取注册表 `HKEY_LOCAL_MACHINE\SOFTWARE\Wow6432Node\Valve\Steam`，其次检查默认路径
- macOS: 检查 `~/Library/Application Support/Steam/`
- 支持通过 `libraryfolders.vdf` 解析所有 Steam 库目录

### F2: DST 客户端安装检测

- 基于 Steam 安装目录，检测 DST 客户端是否已安装
- DST AppID: 322330
- 解析 `appmanifest_322330.acf` 获取安装目录
- 返回完整的游戏可执行文件路径

### F3: DST Dedicated Server 安装检测

- 基于 Steam 安装目录，检测 DST Dedicated Server 是否已安装
- DST Dedicated Server AppID: 343050
- 解析 `appmanifest_343050.acf` 获取安装目录
- 返回完整的服务器可执行文件路径

### F4: 统一检测接口

- 提供 `detect_all()` 一次性检测全部组件
- 返回各组件的安装状态和路径

### F5: 手动指定目录

- 支持手动指定 Steam/DST/DST Server 的安装目录
- 用于自动检测失败时的备选方案

## 错误处理

### 错误码定义

在 `src-tauri/src/utils/` 下新增 `error_code.rs`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    // Steam 相关错误 (1xxx)
    SteamNotFound = 1001,
    SteamPathInvalid = 1002,
    SteamLibraryParseError = 1003,

    // DST 客户端错误 (2xxx)
    DstNotFound = 2001,
    DstManifestParseError = 2002,

    // DST Dedicated Server 错误 (3xxx)
    DstServerNotFound = 3001,
    DstServerManifestParseError = 3002,

    // 通用错误 (9xxx)
    UnknownError = 9999,
}
```

### 错误返回格式

```rust
pub struct DetectionError {
    pub code: ErrorCode,
    pub message: String,
    pub suggestion: String,
}
```

错误信息示例：
- Steam 未安装: "Steam 未安装，请从 Steam 官网下载安装"
- DST 未安装: "饥荒联机版未安装，请通过 Steam 客户端安装"
- DST Server 未安装: "饥荒联机版服务器未安装，请在 Steam 中搜索 'Don't Starve Together Dedicated Server' 并安装"

## 数据结构

### DetectionResult

```rust
pub struct DetectionResult {
    pub steam_path: Option<String>,
    pub dst_path: Option<String>,
    pub dst_executable: Option<String>,
    pub dst_server_path: Option<String>,
    pub dst_server_executable: Option<String>,
    pub errors: Vec<DetectionError>,
}

pub struct ManualPathInput {
    pub steam_path: Option<String>,
    pub dst_path: Option<String>,
    pub dst_server_path: Option<String>,
}
```

### 安装状态分级（方案 B）

为避免"目录存在但可执行文件缺失"这类误判，检测结果采用分级状态，而不是仅返回二元已安装/未安装。

```rust
pub enum InstallLevel {
    NotFound,         // 未发现目标路径
    PathFound,        // 路径存在
    ManifestFound,    // appmanifest 存在且可解析
    ExecutableFound,  // 可执行文件存在（判定为可运行）
}
```

分级判定规则：

- Steam: 至少达到 `PathFound` 才视为已检测到 Steam 安装；若能成功解析 `libraryfolders.vdf`，记录为更高可信状态
- DST / DST Dedicated Server: 仅当达到 `ExecutableFound` 才判定为"已安装可用"
- 当状态低于 `ExecutableFound` 时，保留当前最高等级并写入对应错误码与建议，便于前端给出精确修复引导

建议在返回模型中为 Steam、DST、DST Dedicated Server 分别增加状态字段（如 `steam_level`、`dst_level`、`dst_server_level`），并与路径字段并行返回。

该方案目标是提高诊断可解释性：

- 用户可知道"卡在哪一层"（路径/manifest/可执行文件）
- 前端可按层级展示不同修复建议
- 后续日志与问题排查成本更低

## Tauri IPC 接口

通过 `src-tauri/commands/` 注册命令:

```rust
#[tauri::command]
pub async fn detect_installation() -> Result<DetectionResult, String>;

#[tauri::command]
pub async fn set_manual_path(input: ManualPathInput) -> Result<(), String>;
```

## 检测流程

```
1. detect_steam()
   ├── 读取注册表 (Windows)
   ├── 检查默认路径
   └── 解析 libraryfolders.vdf

2. detect_dst(steam_path)
   ├── 遍历所有库目录
   ├── 查找 appmanifest_322330.acf
   └── 组合完整路径

3. detect_dst_server(steam_path)
   ├── 遍历所有库目录
   ├── 查找 appmanifest_343050.acf
   └── 组合完整路径
```

## 平台支持

| 平台 | Steam 检测 | DST 检测 |
|------|-----------|---------|
| Windows | 注册表 + 默认路径 | appmanifest |
| macOS | 默认路径 | appmanifest |

## 文件结构

```
src-tauri/
├── src/
│   ├── main.rs
│   ├── utils/
│   │   └── error_code.rs    # 新增
│   ├── services/
│   │   └── steam_detection.rs  # 新增
│   └── commands/
│       └── mod.rs           # 新增/修改
```

## 验收标准

- [ ] Windows 平台能正确检测 Steam 安装目录
- [ ] Windows 平台能正确检测 DST 客户端安装目录
- [ ] Windows 平台能正确检测 DST Dedicated Server 安装目录
- [ ] macOS 平台能正确检测 Steam 安装目录
- [ ] macOS 平台能正确检测 DST 客户端安装目录
- [ ] macOS 平台能正确检测 DST Dedicated Server 安装目录
- [ ] 自动检测失败时可手动指定目录
- [ ] 错误信息包含具体缺失组件和安装建议
- [ ] 错误码统一定义并易于扩展
