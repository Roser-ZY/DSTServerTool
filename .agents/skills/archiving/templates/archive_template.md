# {TOPIC}

> **归档日期**: {DATE}
> **归档版本**: {VERSION}
> **相关 Git 提交**: {COMMITS}

---

## 1. 概要需求描述

> 本节基于 FR 文档提炼，简要说明本次任务的背景、目标与核心需求。

### 1.1 背景

{BACKGROUND}

### 1.2 目标

{GOALS}

### 1.3 核心需求

{REQUIREMENTS}

---

## 2. 涉及系统

> 列出本次任务涉及的所有系统模块，说明各系统的职责边界。

| 系统 / 模块 | 层级 | 职责说明 |
|------------|------|---------|
| {SYSTEM_1} | Client / Server / Common | {RESPONSIBILITY_1} |

---

## 3. 功能操作流程

> 描述从玩家视角出发的正常游戏流程，按步骤列出核心操作。

### 3.1 {FEATURE_NAME}

**前置条件：**
- {PRECONDITION}

**操作步骤：**

1. {STEP_1}
2. {STEP_2}
3. {STEP_3}

**结果：**
- {RESULT}

---

## 4. 系统架构图

### 4.1 类图

> 描述核心类的属性、方法及继承/实现关系。

```mermaid
classDiagram
    class ClassName {
        +field: Type
        +method(): ReturnType
    }
    ClassName <|-- SubClass
    ClassName --> DependencyClass
```

### 4.2 类依赖关系图

> 描述模块/包之间的依赖关系。

```mermaid
graph TD
    A[Module A] --> B[Module B]
    A --> C[Module C]
    B --> D[Module D]
```

### 4.3 流程图

> 描述核心功能的执行流程（决策分支、循环等）。

```mermaid
flowchart TD
    Start([开始]) --> Step1[步骤1]
    Step1 --> Decision{判断条件}
    Decision -->|Yes| Step2[步骤2]
    Decision -->|No| Step3[步骤3]
    Step2 --> End([结束])
    Step3 --> End
```

### 4.4 时序图

> 描述各模块/角色之间的交互顺序（适用于客户端-服务端通信等）。

```mermaid
sequenceDiagram
    participant Client as 客户端
    participant Server as 服务端
    participant DataMgr as 数据层

    Client->>Server: 请求操作(params)
    Server->>DataMgr: 查询数据
    DataMgr-->>Server: 返回结果
    Server-->>Client: 同步响应
```

### 4.5 数据流图

> 描述事件、数据等的流向，包含数据初始化、读取、请求、响应、写入等流程均需要独立的数据流。

---

## 5. 其他 UML 模型图

> 根据任务特点，按需补充以下图表类型。

### 5.1 组件图（可选）

> 描述系统组件的组织方式及对外接口。

```mermaid
graph LR
    subgraph 客户端
        A[ComponentA]
        B[ComponentB]
    end
    subgraph 服务端
        C[ComponentC]
    end
    A -->|事件| C
    C -->|回调| B
```

### 5.2 活动图（可选）

> 描述并发流程或复杂业务逻辑的活动分配。

```mermaid
flowchart TD
    subgraph 客户端处理
        C1[预验证] --> C2[本地执行]
    end
    subgraph 服务端处理
        S1[权威验证] --> S2[广播同步]
    end
    C2 -->|网络请求| S1
    S2 -->|回包| C2
```
### 5.3 状态图（可选）

> 描述对象/实体的生命周期状态转换（如武器状态、玩家状态等）。

```mermaid
stateDiagram-v2
    [*] --> Idle: 初始化
    Idle --> Active: 激活触发
    Active --> Processing: 开始处理
    Processing --> Idle: 处理完成
    Active --> Error: 发生错误
    Error --> Idle: 错误恢复
    Idle --> [*]: 销毁
```
---

## 6. 关键实现说明

> 记录实现过程中的重要决策、特殊处理逻辑、性能优化点，或踩坑记录。

### 6.1 {IMPL_POINT_1}

{IMPL_DESCRIPTION_1}

### 6.2 已知限制 / 待优化项

- {KNOWN_ISSUE_1}

---

## 7. 归档文件索引

| 文件 | 类型 | 说明 |
|------|------|------|
| `{FR_DOC}` | FR 文档 | 功能需求说明 |
| `{PLAN_DOC}` | Plan 文档 | 实现计划 |
| `{TOPIC}.md` | 归档总结 | 本文件 |
