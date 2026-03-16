# 验证规则参考

本文件定义 verifying-spec 技能的所有验证规则，按层级组织。验证器在执行时按需加载本文件中的对应规则集。

## 目录

1. [Layer 1: 原始需求一致性](#layer-1-原始需求一致性)
2. [Layer 2: Brainstorm/Explore 一致性](#layer-2-brainstormexplore-一致性)
3. [Layer 3: 文档间交叉一致性](#layer-3-文档间交叉一致性)
4. [Layer 4: Instruction 合规性](#layer-4-instruction-合规性)
5. [通用规则: 编号溯源](#通用规则-编号溯源)

---

## Layer 1: 原始需求一致性

**验证目标**: 所有 SDD 产出文档是否忠实于用户提供的原始需求。

**原始需求来源**（按优先级）:
1. 用户提供的 PRD / SRS / 需求文档文件
2. 对话上下文中用户确认的需求描述
3. 用户通过外部渠道（邮件、Slack、文档链接等）提供的补充说明

### 规则 L1-001: 功能点完整覆盖

**检查方法**: 从原始需求中提取所有功能点 → 逐条在 FR/FI 文档中匹配。

| 判定 | 条件 |
|------|------|
| PASS | 原始需求中每个功能点在 FR 文档中都有对应编号 |
| FAIL | 存在原始需求功能点未出现在任何 FR 文档中 |
| WARN | FR 文档中存在原始需求未提及的功能点（可能是 brainstorm 新增，需标注来源） |

**输出格式**:
```
L1-001 功能点完整覆盖
  ✅ 原始需求功能点 N 项，FR 文档覆盖 M 项
  ❌ 遗漏: [原始需求 §X.Y "功能描述"] → FR 文档中未找到对应
  ⚠️ 新增: [FR-0003] 在原始需求中未提及 → 来源: brainstorm §2.1
```

### 规则 L1-002: 非功能需求保留

**检查方法**: 从原始需求中提取 NFR → 在 design/plan 文档中验证是否体现。

| 判定 | 条件 |
|------|------|
| PASS | 原始需求中的性能/安全/可用性等 NFR 在设计文档中有对应章节或约束 |
| FAIL | NFR 被完全忽略（无任何提及） |
| WARN | NFR 被提及但未量化（如原始需求写"响应 < 2s"，设计文档只写"低延迟"） |

### 规则 L1-003: 范围边界一致

**检查方法**: 对比原始需求的 "In Scope / Out of Scope" 与 FR/design 文档的功能边界。

| 判定 | 条件 |
|------|------|
| PASS | 文档的功能边界与原始需求一致 |
| FAIL | 原始需求明确标记 Out of Scope 的功能出现在 FR 中 |
| FAIL | 原始需求明确标记 In Scope 的功能未出现在 FR 中 |

### 规则 L1-004: 术语一致性

**检查方法**: 提取原始需求中的核心术语 → 在 FR/design/plan 中匹配。

| 判定 | 条件 |
|------|------|
| PASS | 核心业务术语在所有文档中统一 |
| WARN | 同一概念在不同文档中使用不同名称（如原始需求用"订单"、FR 用"工单"） |

---

## Layer 2: Brainstorm/Explore 一致性

**前置条件**: 仅当 SDD 流程包含 brainstorm 或 explore 阶段时执行。

**Brainstorm 产出位置**: `docs/<topic>/YYYY-MM-DD-<topic>-design.md`

### 规则 L2-001: Brainstorm 结论完整落地

**检查方法**: 从 brainstorm 文档提取已确认的结论 → 在 FR/FI 文档中逐条匹配。

| 判定 | 条件 |
|------|------|
| PASS | brainstorm 中每个确认的功能点/设计决策都在 FR/design 文档中体现 |
| FAIL | brainstorm 确认的结论在后续文档中丢失 |
| FAIL | FR/design 文档中出现 brainstorm 明确否决的方案 |

### 规则 L2-002: 功能边界与 Brainstorm 共识一致

**检查方法**: 对比 brainstorm 中确定的"不包含项" → FR/design 文档不应包含这些功能。

| 判定 | 条件 |
|------|------|
| PASS | brainstorm 标记为 YAGNI / 不包含 的功能未出现在 FR 中 |
| FAIL | brainstorm 明确排除的功能出现在 FR 中 |

### 规则 L2-003: 方案选择一致

**检查方法**: brainstorm 中讨论了多个方案并选择了一个 → design/plan 采用的方案应与选择一致。

| 判定 | 条件 |
|------|------|
| PASS | design/plan 采用了 brainstorm 中选定的方案 |
| FAIL | design/plan 采用了 brainstorm 中被否决的方案 |
| WARN | design/plan 采用了 brainstorm 中未讨论的新方案（需标注决策来源） |

### 规则 L2-004: Explore 发现事项反映

**检查方法**: 如有 explore 阶段的发现（技术可行性、现有代码约束等），验证 design/plan 是否反映。

| 判定 | 条件 |
|------|------|
| PASS | explore 发现的技术约束在 design 文档的"约束"章节中体现 |
| WARN | explore 发现事项在 design 中未提及 |

---

## Layer 3: 文档间交叉一致性

**验证目标**: FR、FI、design、plan 文档之间的内容互相一致。

### 规则 L3-001: FR → Design 映射完整性

**检查方法**: 每个 FR-xxxx 在 design 文档中是否有对应的 DM-FR-xxxx 设计映射。

| 判定 | 条件 |
|------|------|
| PASS | 每个 FR-xxxx 都有对应的 DM-FR-xxxx |
| FAIL | 存在 FR-xxxx 无对应设计映射（设计缺失） |
| FAIL | 存在 DM-FR-xxxx 无对应 FR-xxxx（孤立设计） |

### 规则 L3-002: FR → Plan 任务覆盖

**检查方法**: 每个 FR 功能点在 plan 文档中是否有对应的实现任务。

| 判定 | 条件 |
|------|------|
| PASS | 每个 P0/P1 FR 在 plan 中有至少一个 Task 覆盖 |
| FAIL | 存在 P0 FR 无对应 plan Task |
| WARN | 存在 P1 FR 无对应 plan Task |

### 规则 L3-003: Design → Plan 技术一致性

**检查方法**: plan 中的技术方案是否与 design 文档一致。

| 判定 | 条件 |
|------|------|
| PASS | plan 中引用的技术栈、架构模式与 design 一致 |
| FAIL | plan 使用了 design 中未描述的架构或组件 |
| WARN | plan 对 design 方案做了简化但未说明原因 |

### 规则 L3-004: FR ↔ FI 对应关系

**检查方法**: 每个 FR-xxxx 有对应的 FI-xxxx（如果 SDD 流程包含 FI 生成）。

| 判定 | 条件 |
|------|------|
| PASS | FR-xxxx 和 FI-xxxx 编号一一对应 |
| FAIL | 存在 FR 无对应 FI |
| FAIL | FI 中的功能描述与对应 FR 矛盾 |

### 规则 L3-005: 接口定义一致

**检查方法**: design/plan 中提及的 API 接口、数据结构定义是否一致。

| 判定 | 条件 |
|------|------|
| PASS | 同一接口在不同文档中的路径、方法、参数、返回值描述一致 |
| FAIL | 同一接口在不同文档中描述矛盾（如 design 写 POST，plan 写 PUT） |

---

## Layer 4: Instruction 合规性

**前置条件**: 仅当 SDD 流程涉及已有 instruction 文档时执行。

**Instruction 来源**:
- `creating-instruction` 的产出: `docs/<topic>/archive/instructions/<topic>_instruction.md`
- `creating-autonomous-instruction` 的产出: `docs/<topic>/archive/autonomous_instructions/<topic>_instruction.md`
- 用户直接指定的 instruction 文件路径

### 规则 L4-001: 架构层级规范遵循

**检查方法**: 从 instruction 提取架构层级规范 → 在 design/plan 文档中验证是否遵循。

| 判定 | 条件 |
|------|------|
| PASS | design 中的模块分层与 instruction 规定的层级规范一致 |
| FAIL | design 违反 instruction 中"严格禁止"的依赖模式 |
| WARN | design 引入了 instruction 未覆盖的新层级 |

### 规则 L4-002: 模块扩展规范遵循

**检查方法**: 从 instruction 提取扩展规范（如工厂注册、事件规范）→ 在 design/plan 中验证。

| 判定 | 条件 |
|------|------|
| PASS | 新增模块/功能遵循 instruction 中的扩展流程 |
| FAIL | 新增模块跳过了 instruction 中标记为"强制"的扩展步骤 |

### 规则 L4-003: 命名与目录规范遵循

**检查方法**: 从 instruction 提取命名/目录约定 → 在 design/plan 中验证文件路径和命名。

| 判定 | 条件 |
|------|------|
| PASS | plan 中的文件路径和命名遵循 instruction 规范 |
| FAIL | plan 中的文件放置违反 instruction 的目录结构规范 |
| WARN | 存在 instruction 未覆盖的新目录（需要更新 instruction） |

### 规则 L4-004: 自治 Instruction 输入要求满足

**检查方法**: 仅当使用 `creating-autonomous-instruction` 产物时，验证 FR/design/plan 是否提供了 instruction 中定义的所有必填输入。

| 判定 | 条件 |
|------|------|
| PASS | instruction 中"开发输入要求"的所有必填项在文档中有对应内容 |
| FAIL | 存在必填输入未在文档中体现 |

### 规则 L4-005: 依赖模块规范遵循

**检查方法**: instruction 中记录的依赖模块 → 在 design/plan 中是否正确引用。

| 判定 | 条件 |
|------|------|
| PASS | design/plan 引用的依赖在 instruction 依赖清单中 |
| WARN | design/plan 引入了 instruction 未记录的新依赖（需要更新 instruction） |
| FAIL | design/plan 引用了不存在的依赖路径 |

---

## 通用规则: 编号溯源

适用于所有层级。

### 规则 G-001: 编号唯一性

每个编号（FR-xxxx、FI-xxxx、DM-FR-xxxx、TC-xxxx）在其文档集中唯一，无重复。

### 规则 G-002: 编号连续性

编号序列无跳号（FR-0001, FR-0002, FR-0003...）。跳号需标注原因（如 FR-0002 被废弃）。

### 规则 G-003: 交叉引用有效性

文档中引用的编号（如 plan 引用 FR-0003）在被引用文档中确实存在。

### 规则 G-004: 优先级一致性

同一 FR 在不同文档中的优先级标记一致（不允许 FR 文档标 P0、plan 中标 P2）。
