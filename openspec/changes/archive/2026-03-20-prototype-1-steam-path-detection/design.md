## Context

The app manages DST server workflows, but currently relies on manual Steam path input. This introduces setup friction and frequent invalid path errors. The change spans frontend UX, shared TypeScript types, and Tauri Rust backend path probing, so a cross-layer design is needed.

Constraints:
- Must work in Tauri desktop context on macOS/Windows.
- Must keep manual path override available when auto-detection is unavailable.
- Must produce deterministic validation results for downstream DST mod update logic.

当前 `src-tauri/src/services/path/path_detection.rs` 中的 Steam 路径检测仅通过硬编码的默认路径列表探测：
- Windows: `C:\Program Files (x86)\Steam`, `C:\Program Files\Steam`
- macOS: `~/Library/Application Support/Steam`

当用户将 Steam 安装在其他位置（如 D:\Steam）或使用 Steam 库文件夹功能配置了额外的游戏库时，现有实现无法找到正确的路径。此外，在 macOS 上当 Steam 未安装时，路径检测可能错误地返回无关目录（如 `~/Documents/New Project`）作为 Steam 根目录，因为现有验证仅检查目录是否存在而不检查内部 Steam 专属文件标记。

## Goals / Non-Goals

**Goals:**
- Provide a single IPC endpoint to detect Steam install path candidates by OS and validate structure.
- Expose detection result in frontend with clear status: detected, invalid, not found.
- Allow user override and re-validation before using path in DST flows.
- Reuse validated path as default source for mod update capability.
- 支持通过 Windows 注册表查询 Steam 安装路径
- 支持解析 `libraryfolders.vdf` 发现所有配置的 Steam 库目录
- 增强 Steam 根目录验证，防止 macOS 等平台上误报无关目录为 Steam 根目录
- 自动探测时在未找到有效 Steam 根目录时返回 `not_found`
- 保持向后兼容，不破坏现有手动路径输入功能

**Non-Goals:**
- 不实现 SteamCMD 或 Steam API 调用
- 不修改前端 UI（当前实现已支持显示路径来源）
- Managing Steam login/account state.
- Auto-installing Steam or modifying Steam directories.
- Detecting every non-standard portable install path beyond bounded probing rules.

## Decisions

### 1. Add dedicated Tauri command `detect_steam_path`

- Rationale: centralizes platform-specific filesystem logic in Rust where OS APIs and path handling are reliable.
- Alternative: frontend-only detection via JavaScript APIs; rejected because browser-side access is limited and less secure in Tauri.

### 2. Return structured response instead of plain string

- Proposed shape: `{ status, path, source, validationErrors }`.
- Rationale: frontend can render actionable UX states without guessing failure reasons.
- Alternative: return nullable path; rejected due to poor diagnosability.

### 3. Introduce a small SteamPath settings section in existing server setup/mod update UI

- Component structure:
  - `SteamPathCard` (status + actions)
  - `SteamPathInput` (manual override)
  - Existing view orchestrates invoke/retry/save.
- Rationale: keeps change isolated and composable while preserving current layout.

### 4. Define a conservative validation rule set

- Validate path exists and contains expected Steam directories/files used by DST workflows.
- Rationale: avoid false positives that break mod update later.
- Alternative: accept any existing directory; rejected as too permissive.

### 5. Windows 注册表查询

**Decision**: 使用 Rust 内置的 Windows API (std::os::windows::ffi) 读取注册表，而非引入 `winreg` crate。

**Rationale**: 
- 减少外部依赖，降低二进制体积
- 仅需读取单个注册表项，实现简单
- 使用 `HKEY_LOCAL_MACHINE` + `SOFTWARE\WOW6432Node\Valve\Steam` (64-bit 环境下的标准路径)

### 5.1 按平台区分根目录验证（修正 macOS 真实布局）

**Decision**: 根目录验证改为平台感知，不再将 Windows 目录结构硬套到 macOS：
- Windows：继续校验 `steamapps/common`、`steamapps/workshop` + Steam 标记文件
- macOS：优先使用 `registry.vdf`、`Steam.AppBundle`、`config/libraryfolders.vdf` 等真实布局标记作为根目录判定依据

**Rationale**: macOS 的 Steam 目录与 Windows 不同。复用 `steamapps/common` 与 `steamapps/workshop` 的硬编码规则，会导致已安装 Steam 也无法通过验证，或错误地将无关目录作为候选根目录。macOS `Steam.AppBundle` 必须包含 `Steam` 子目录作为真实安装标识。

### 6. libraryfolders.vdf 解析

**Decision**: 实现简单的 VDF 文本解析器，提取 `path` 字段。

**Rationale**:
- VDF 格式简单（类 INI 结构），无需完整解析器
- 只需提取 `libraryfolders` 下的 `path` 键值
- 示例结构:
  ```
  "libraryfolders"
  {
    "0"
    {
      "path" "D:\\Games\\Steam"
      ...
    }
  }
  ```

### 7. 路径探测顺序

**Decision**: 按以下顺序探测：
1. 用户手动指定的路径（直接验证）
2. Windows 注册表
3. 默认安装目录
4. 检测到的任何 Steam 目录下的 `libraryfolders.vdf`

**Rationale**: 注册表通常包含最准确的安装路径，应优先于默认路径检查。

### 8. 模块结构

**Decision**: 将 VDF 解析抽取为独立模块 `library_folders.rs`。

**Rationale**:
- 保持 `path_detection.rs` 职责单一
- 便于单独测试 VDF 解析逻辑
- 未来可扩展支持其他 VDF 文件解析

### 9. 自动探测与手动输入的返回语义分离

**Decision**: 
- 手动输入路径：验证失败时返回 `invalid`，附带具体缺失的 Steam 标记信息
- 自动探测路径：验证失败时继续探测，最终返回 `not_found`（不暴露无效猜测路径）

**Rationale**: 手动输入时用户需要知道哪里输错了；自动探测时应该"找不到就直说找不到"，而非返回一个误导性的路径。

### 10. Prototype-1 闭环测试入口设计

**Decision**: 在 `tests/prototype-1/` 下建立单入口测试方案，由统一入口根据目标平台分流执行对应测试用例。
- 测试入口仅一个
- Windows 分支覆盖：注册表检测、默认安装目录检测、libraryfolders.vdf 检测
- macOS 分支覆盖：默认安装目录检测、libraryfolders.vdf 检测、平台真实布局验证

**Rationale**: 用户希望"使用时只提供一个测试入口"，同时又要在不同平台自动执行不同测试内容。单入口 + 平台分流是最小复杂度方案。

### 10.1 Mock 目录与真实安装优先级

**Decision**: 在 `tests/prototype-1/mock/` 下提供各平台虚拟 Steam 目录（Windows/macOS），并定义优先级：
- 若当前系统能探测到真实 Steam 安装，优先使用真实路径进行"找到路径"测试
- 仅当当前系统未探测到真实安装时，才启用对应平台 mock fixture 参与"找到路径"测试

**Rationale**: 避免 mock 覆盖真实环境结果，保证测试语义与生产探测一致，同时为无 Steam 安装环境提供可执行测试闭环。

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| 注册表读取在某些 Windows 配置下失败 | 失败时静默降级到默认路径探测 |
| VDF 文件格式因 Steam 版本更新而变化 | 使用宽松的文本解析，忽略未知字段 |
| 用户无注册表读取权限 | 默认路径探测作为兜底方案 |
| 非 Steam 安装（如 Humble Bundle） | 保持手动路径输入功能 |
| Steam 专属标记选择过窄可能拒绝合法安装 | 优先选择稳定的 Steam 配置文件，覆盖已知平台布局 |
| macOS 无关目录误报为 Steam 根目录 | 使用 Steam 专属文件标记验证（含 Steam.AppBundle/Steam 子目录检查） |
| 跨平台测试入口逻辑复杂导致可维护性下降 | 统一入口仅负责平台分发，具体断言拆分为平台子模块 |
| mock fixture 与真实目录行为不一致导致误判 | 定义最小真实布局基线并在 README 中显式记录差异与假设 |
| [OS path variance] Some installations may live outside default probe locations | Mitigation: keep manual override and explicit validation errors. |
| [False negatives] Conservative validation may reject unusual but valid layouts | Mitigation: allow manual retry + document accepted structure. |
| [Coupling with mod update] New default behavior could change user expectation | Mitigation: only auto-fill when status is valid; never remove manual control. |

## Migration Plan

1. **Phase 1**: 实现 `library_folders.rs` 模块，添加单元测试
2. **Phase 2**: 修改 `path_detection.rs`，集成注册表查询和 VDF 探测
3. **Phase 3**: 添加 Steam 根目录验证标记检查
4. **Phase 4**: 分离自动探测与手动输入的返回语义
5. **Phase 5**: 添加 macOS 误报和未安装 Steam 的回归测试
6. **Phase 6**: 运行现有测试，确保向后兼容
7. **Phase 7**: 在用户的实际环境（D:\Steam）验证功能
8. **Phase 8**: 建立 `tests/prototype-1` 闭环测试目录、README 与单入口平台分流测试
9. **Phase 9**: 在各平台验证测试入口可独立运行且仅覆盖后端能力
10. **Phase 10**: 增加 `tests/prototype-1/mock` 跨平台虚拟 Steam fixture，并实现"仅未找到真实安装时启用"策略
11. Add backend IPC command and unit-testable validation helpers.
12. Add typed frontend IPC wrapper and state model.
13. Integrate UI controls into relevant setup/mod update view.
14. Gate default mod-update source path on `status=valid` only.
15. Rollback strategy: disable new UI usage and fallback to current manual-only path flow.

**Rollback**: 通过 Cargo workspace 可以快速回退到旧版本二进制。

## Open Questions

- Should we persist the last valid detected path globally or per DST cluster profile?
- Do we need a user-facing "probe common paths" action beyond automatic probe on entry?
