---
name: verifying-spec
description: SDD 流程中的文档规格验证技能。在以下场景触发：(1) brainstorm/explore 完成后、生成 FR/FI 之前，验证 brainstorm 结论与原始需求一致；(2) FR/FI/design/plan 文档生成后、开始实现之前，验证所有文档间的交叉一致性；(3) 用户手动请求"验证规格"/"校验文档"/"检查一致性"时。验证范围为纯文档对文档（不涉及代码），检查原始需求覆盖、brainstorm 结论落地、文档间编号映射、instruction 规范合规等，输出结构化验证报告文件。支持 openspec/ 和 docs/<topic>/ 两种目录结构。
---

# Verifying Spec — 文档规格验证

## 定位

验证 SDD 流程中各阶段产出文档与用户原始需求之间的**一致性**，以及文档彼此之间的**交叉一致性**。

**本技能只做文档对文档验证，不涉及代码。** 代码与文档的一致性由 `verifying-implementation` 技能负责。

---

## 前置信息收集

执行验证前，**必须**确认以下信息：

| 信息项 | 必填 | 说明 |
|--------|------|------|
| **原始需求来源** | 是 | PRD/SRS 文件路径，或指明"对话上下文" |
| **规格文档根目录** | 是 | `openspec/` 或 `docs/<topic>/` |
| **验证触发时机** | 是 | brainstorm 后 / 文档完成后 / 手动 |
| **brainstorm 文档** | 条件 | 如流程包含 brainstorm 阶段，提供路径 |
| **instruction 文档** | 条件 | 如流程涉及 instruction，提供路径 |
| **报告输出路径** | 否 | 默认 `{规格文档根目录}/verification-report-{DATE}.md` |

如用户未提供必填项，停止并提示补充。条件项根据 SDD 流程是否包含对应阶段决定。

---

## 验证分层模型

验证按 4 层 + 通用规则执行，每层可独立启用/跳过：

```
Layer 1: 原始需求一致性          ← 始终执行
Layer 2: Brainstorm/Explore 一致性 ← 仅当有 brainstorm/explore 阶段
Layer 3: 文档间交叉一致性         ← 始终执行
Layer 4: Instruction 合规性       ← 仅当涉及 instruction
通用规则: 编号溯源               ← 始终执行
```

每条规则完整定义见 `references/verification-rules.md`。

---

## 执行流程

### Step 1: 确定验证范围

根据 SDD 流程所处阶段，决定启用哪些层：

| 触发时机 | 启用层 |
|----------|--------|
| brainstorm 后、FR 生成前 | L1 + L2 + 通用 |
| 文档全部完成后、实现前 | L1 + L2(条件) + L3 + L4(条件) + 通用 |
| 手动触发 | 全部（按条件跳过不适用的层） |

### Step 2: 收集文档

按以下顺序定位并读取文档：

**2.1 原始需求**
- 从用户指定路径读取 PRD/SRS 文件
- 或从对话上下文中提取用户确认的需求要点

**2.2 规格文档**（从 `openspec/` 或 `docs/<topic>/` 读取）
- FR 文档: `FR-xxxx_*.md` / `FR-xxxx_*.yaml`
- FI 文档: `FI-xxxx_*.md` / `FI-xxxx_*.yaml`（如有）
- Design 文档: `*-design.md` / `*-设计文档.md`
- Plan 文档: `*-plan.md` / `*.md`（含 Task 结构的文档）

**2.3 Brainstorm 文档**（如适用）
- 位置: `docs/<topic>/YYYY-MM-DD-<topic>-design.md`
- 或用户指定路径

**2.4 Instruction 文档**（如适用）
- 位置: `docs/<topic>/archive/instructions/` 或 `docs/<topic>/archive/autonomous_instructions/`
- 或用户指定路径

### Step 3: 逐层执行验证

按 L1 → L2 → L3 → L4 → 通用 顺序执行。每条规则的判定逻辑见 `references/verification-rules.md`。

**关键操作**:

1. **L1 原始需求一致性** — 从原始需求提取功能点清单 → 逐条在 FR 中匹配 → 检查 NFR 保留 → 检查范围边界 → 检查术语统一
2. **L2 Brainstorm 一致性** — 从 brainstorm 提取确认结论 → 在 FR/design 中匹配 → 检查被否决方案未混入 → 检查功能边界共识
3. **L3 文档间交叉** — FR ↔ Design 编号映射 → FR → Plan 任务覆盖 → Design ↔ Plan 技术一致 → FR ↔ FI 对应 → 接口定义一致
4. **L4 Instruction 合规** — 架构层级遵循 → 扩展规范遵循 → 命名/目录规范 → 自治 instruction 输入满足 → 依赖模块引用正确
5. **通用** — 编号唯一 → 编号连续 → 交叉引用有效 → 优先级一致

### Step 4: 生成验证报告

使用 `references/report-template.md` 模板，生成验证报告文件。

**报告文件**:
- 路径: `{规格文档根目录}/verification-report-{YYYY-MM-DD}.md`
- 包含: 概要、每层每规则的判定结果、问题汇总表、后续动作

### Step 5: 判定与建议

| 结论 | 条件 | 建议 |
|------|------|------|
| **PASS** | 0 FAIL, 0 WARN | 可进入下一 SDD 阶段 |
| **PASS WITH WARNINGS** | 0 FAIL, >=1 WARN | 建议评估 WARN 项，可继续 |
| **FAIL** | >=1 FAIL | 必须修复 FAIL 项后重新验证 |

FAIL 时，在报告末尾列出具体修复建议和涉及的文件清单。

---

## 与 SDD 工作流的集成

本技能可在以下位置被 SDD 流程调用：

```
brainstorm → [verifying-spec L1+L2] → creating-frs → creating-frs-and-fis
→ design → plan → [verifying-spec L1+L2+L3+L4] → implementation
→ verifying-implementation（代码验证，非本技能）
```

也可由用户在任意时刻手动触发，传入文档路径即可。

---

## 冲突解决原则

当文档之间存在冲突时：

| 优先级 | 来源 | 说明 |
|--------|------|------|
| 1 (最高) | 用户对话中明确确认的需求 | 实时沟通优先 |
| 2 | 原始 PRD/SRS 文件 | 正式文档 |
| 3 | Brainstorm 确认结论 | 经讨论确认的设计 |
| 4 | Instruction 规范 | 已有架构约束 |
| 5 (最低) | FR/Design/Plan 文档 | 待验证对象 |

发现冲突时：
- 如果高优先级来源明确，按其修正低优先级文档并标记
- 如果无法判定，**立即向用户提问**，不得自行假设

---

## 输出规范

- **报告格式**: Markdown，使用 `references/report-template.md` 结构
- **文件命名**: `verification-report-{YYYY-MM-DD}.md`
- **输出位置**: 规格文档根目录下
- **严重性标记**: PASS(✅) / FAIL(❌) / WARN(⚠️) / SKIP(⏭️)
