---
name: creating-frs
description: 本 Skill 在 superpower:brainstorming） brainstorming） 技能完成后触发。必须基于 brainstorming） 阶段已经形成的上下文结论，结合用户提供的原始文档进行功能点拆解。
license: MIT
---

## Usage Preconditions

- 本 Skill **只能在 superpower:brainstorm 已完成后触发**
- 必须已在 brainstorming 阶段形成以下上下文结论：
  - 功能点列表
  - 每个功能点的核心意图
  - 功能边界与不包含项的共识
  - 用户**明确**给出了任务需求的名称
- 若上述结论不存在或不完整，必须中止执行并提示用户补充

---

## Inputs

- 用户提供的原始文档（PRD / 需求说明 / 设计说明等）
- brainstorming 阶段形成的上下文结论（作为隐式输入）

---

## Responsibilities

为每一个明确的功能点分别生成一份 Feature Spec。
每个功能点必须同时生成：
    1）一份 FR-xxxx_<name>.md（中文可读规格文档）。
    MD 文件必须严格基于 frs-template.yaml 模板生成，
    严禁脱离 brainstorming 阶段结论或模板规范自行推断或扩展功能点。
每个功能点单独创建 FR-xxxx 目录管理 md 文件。
所有功能点目录维护在统一的 docs/{**用户提供的任务需求名称**}/ 目录下。

1. 生成的 Feature Spec **必须**涵盖用户提供的原始文档和 brainstorming 对原始文档的补充、修正等**全部**内容，**遗漏任何内容都是错误的**
2. 必须原始文档和 brainstorming 阶段讨论结论中的功能点进行拆解
3. 不得新增、合并、拆分或推断 brainstorming 中未明确的功能点
4. 为每一个功能点分配唯一编号：
   - FR-0001, FR-0002, …
5. 每个功能点必须生成且仅生成以下文件：
   - FR-xxxx_<feature_name>.md
6. 每个功能点的文件中必须包含实现方案（implementation solution）。

---

## Conflicts

如果原始文档和 brainstorming 文档（brainstorming 文档默认为 `docs/plans/YYYY-MM-DD-<topic>-design.md`）存在冲突：
- 如果 brainstorming 文档中确认已经讨论过该冲突，以 brainstorming 为准。
- 否则进行询问。

## YAML Generation Rules（强制）

- **所有 Feature MD 文件必须严格基于 `frs-template.yaml` 模板生成**
- 不允许：
  - 缺失模板字段
  - 新增未在模板中定义的字段
  - 改变字段层级结构
- 所有字段值必须使用中文

---

## Markdown Rendering Rules

- Markdown 文件必须完全由模板 yaml 渲染生成
- 不允许在 Markdown 中引入 yaml 中不存在的新信息
- 标题格式必须为：
  - `# FR-xxxx 功能点名称`
- Markdown 内容结构需与 frs-template.yaml 字段一一对应
- 行文风格为工程化规格说明，避免口语化描述
- 全文中文，在 Markdown 中将 Yaml 字段翻译为中文。

---

## Output Structure

- 每个功能点独立输出一组文件：
  - FR-xxxx_<feature_name>.md
- 不同功能点之间：
  - 不得隐式引用未声明的依赖
- 文档中的所有图表，均使用 mermaid 描述。

---

## Failure Conditions

在以下任一情况下，必须中止执行并明确说明原因：

- brainstorm 阶段结论缺失或不完整
- 功能点边界存在歧义，无法映射为独立 FRS
- 无法严格按照 frs-template.yaml 模板生成 Markdown