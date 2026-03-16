---
  name: creating-frs
  description: 本 Skill 在 superpower:brainstorm 阶段完成后触发。必须基于 brainstorm 阶段已经形成的上下文结论，对用户提供的原始文档进行功能点拆解。
  license: MIT
---

## Usage Preconditions

- 本 Skill **只能在 superpower:brainstorm 已完成后触发**
- 必须已在 brainstorm 阶段形成以下上下文结论：
  - 功能点列表
  - 每个功能点的核心意图
  - 功能边界与不包含项的共识
  - 用户**明确**给出了任务需求的名称
- 若上述结论不存在或不完整，必须中止执行并提示用户补充

---

## Inputs

- 用户提供的原始文档（PRD / 需求说明 / 设计说明等）
- superpower:brainstorm 阶段形成的上下文结论（作为隐式输入）

---

## Responsibilities

为每一个明确的功能点分别生成一份 Feature Spec。

每个功能点必须同时生成以下四个文件：
    1）一份结构化的 FR-xxxx.yaml（作为需求唯一规格真源）。
    2）一份对应的 FR-xxxx_<name>.md（中文可读规格文档）。
    3）一份结构化的 FI-xxxx.yaml（作为实现唯一规格真源）。
    4）一份对应的 FI-xxxx_<name>.md（中文可读实现文档）。
    YAML 文件必须严格基于 frs-template.yaml 模板生成，
    严禁脱离 brainstorm 阶段结论或模板规范自行推断或扩展功能点。
每个功能点单独创建 FR-xxxx 目录管理 yaml 和 md 文件。
所有功能点目录维护在统一的 docs/{**用户提供的任务需求名称**}/ 目录下。

1. 严格依据 brainstorm 阶段已经确认的功能点进行拆解
2. 不得新增、合并、拆分或推断 brainstorm 中未明确的功能点
3. 为每一个功能点分配唯一编号：
   - FR-0001, FR-0002, …
   - FR-0001 ↔ FI-0001
4. 每个功能点必须生成且仅生成以下两个文件：
   - FR-xxxx.yaml
   - FR-xxxx_<name>.md
   - FI-xxxx.yaml
   - FI-xxxx_<name>.md

---

## YAML Generation Rules（强制）

- FR YAML：
  - 必须严格基于 `frs-template.yaml`
  - 只描述功能需求，不包含实现方案
- FI YAML：
  - 必须严格基于 `fis-template.yaml`
  - 必须引用对应 FR（feature_ref）
- 所有 YAML 字段值必须为中文
- YAML 是唯一事实来源（Source of Truth）

---

## Markdown Rendering Rules

- Markdown 文件必须完全由对应的 FR-xxxx.yaml 渲染生成
- 不允许在 Markdown 中引入 YAML 中不存在的新信息
- 标题格式必须为：
  - `# FR-xxxx 功能点名称`
- Markdown 内容结构需与 frs-template.yaml 字段一一对应
- 行文风格为工程化规格说明，避免口语化描述。
- Use writing-clearly-and-concisely skill if available
- 全文中文

---

## Output Structure

- 每个功能点独立输出一组文件，顺序如下：
  1. FR-xxxx_<feature_name>.yaml
  2. FR-xxxx_<feature_name>.md
- 不同功能点之间：
  - 不得共享 YAML 文件
  - 不得隐式引用未声明的依赖

---

## Failure Conditions

在以下任一情况下，必须中止执行并明确说明原因：

- brainstorm 阶段结论缺失或不完整
- 功能点边界存在歧义，无法映射为独立 FRS
- 无法严格按照 frs-template.yaml 模板生成 YAML