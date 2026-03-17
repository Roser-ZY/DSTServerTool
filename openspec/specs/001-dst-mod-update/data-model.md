# Data Model: DST-1 模组更新检测与替换

## Entities

### SteamInstall
- **Fields**: `root_path`, `steamapps_common_path`, `steamapps_workshop_path`
- **Validation**: `root_path` 必须存在且包含 `steamapps` 结构。

### ServerMod
- **Fields**: `workshop_id`, `server_mod_path`, `last_modified_at`
- **Validation**: `workshop_id` 必须为数字；路径存在且以 `workshop-` 前缀命名。

### SourceMod
- **Fields**: `workshop_id`, `source_path`, `source_type` (workshop | game_mods), `last_modified_at`
- **Validation**: `source_path` 存在；`workshop_id` 与服务器模组匹配。

### UpdateResult
- **Fields**: `workshop_id`, `status` (updated | unchanged | missing | failed), `reason`, `source_path`, `server_mod_path`
- **Validation**: `status` 必须为预定义枚举值。

## Relationships

- **SteamInstall** 1 -> N **SourceMod**
- **ServerMod** 1 -> 0..N **SourceMod** (按 `workshop_id` 匹配)
- **ServerMod** 1 -> 1 **UpdateResult**

## State Transitions

- **ServerMod** -> **UpdateResult.status**:
  - `missing`: 未找到任何来源模组
  - `unchanged`: 找到来源但更新时间不更新
  - `updated`: 来源更新时间更新且完成替换
  - `failed`: 替换过程中发生错误
