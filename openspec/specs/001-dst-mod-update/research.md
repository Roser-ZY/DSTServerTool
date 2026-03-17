# Research: DST-1 模组更新检测与替换

## Decision 1: 更新判定依据
**Decision**: 使用模组目录的最后修改时间作为更新判定依据。  
**Rationale**: Windows 环境下普遍可获取，且无需额外依赖即可判断更新先后。  
**Alternatives considered**: 使用 Steam API 或模组元数据文件进行版本比对（超出当前范围且增加依赖）。

## Decision 2: 来源路径优先级
**Decision**: 当来源模组同时存在于 `steamapps/workshop` 与 `Dont Starve Together/mods` 时，选择更新时间更晚的来源。  
**Rationale**: 符合“更新更晚即更可信”的用户预期，避免硬编码单一路径优先级。  
**Alternatives considered**: 固定优先级（始终优先 `workshop` 或 `DST/mods`），但可能错过本地更新。

## Decision 3: 影响范围与安全性
**Decision**: 仅更新服务器 `mods` 目录中已有的 `workshop-<id>` 目录，不新增不在列表中的模组。  
**Rationale**: 降低误拷贝风险，保证更新行为可控且可回溯。  
**Alternatives considered**: 自动新增来源存在但服务器未安装的模组（可能引入意外内容）。

## Decision 4: 失败处理与可追溯性
**Decision**: 对每个模组输出状态（已更新、未更新、未找到、失败）与原因。  
**Rationale**: 满足管理员快速定位问题的需求，避免“静默失败”。  
**Alternatives considered**: 仅汇总统计，不输出逐项结果（不利于排查）。
