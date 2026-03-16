---
name: {TOPIC}-instruction
description: {TRIGGER_CONDITION}
---

# {TOPIC} 开发指导文档

> **生成日期**: {DATE}
> **适用范围**: 与{TOPIC}相关的所有后续开发任务
> **更新要求**: 当{TOPIC}相关代码发生重大架构变更时需更新本文档
> **使用方式**: 开发过程中作为架构规范和扩展指导的参考文档

---

## 1. 任务概述

### 1.1 功能描述

{TOPIC_DESCRIPTION}

### 1.2 核心职责

{CORE_RESPONSIBILITIES}

### 1.3 系统定位

{SYSTEM_POSITIONING}

---

## 2. 系统开发规范

> **⚠️ 重要：本节内容为强制遵循的开发规范，所有后续相关开发必须严格按照此规范执行。**

### 2.1 架构层级规范

{ARCHITECTURE_LAYERS}

**层级调用原则：**
- {LAYER_CALL_RULES}

**禁止模式：**
- {FORBIDDEN_PATTERNS}

### 2.2 模块扩展规范

{MODULE_EXTENSION_RULES}

**新增模块必须遵循：**
1. {EXTENSION_RULE_1}
2. {EXTENSION_RULE_2}
3. {EXTENSION_RULE_3}

### 2.3 核心设计模式

{DESIGN_PATTERNS_RULES}

### 2.4 关键机制规范

{KEY_MECHANISMS_RULES}

---

## 3. 依赖模块清单

> **⚠️ 每个依赖都必须能通过本文档信息直接定位，不需要二次搜索。**

### 3.1 仓库内依赖

| 模块名称 | 路径（相对仓库根目录） | 说明 |
|---------|----------------------|------|
| {REPO_DEP_ROWS} |

### 3.2 第三方依赖

**需要下载安装：**

| 库名称 | 版本要求 | 安装方式 | 说明 |
|-------|---------|---------|------|
| {THIRD_PARTY_DOWNLOAD_ROWS} |

**本地已有：**

| 库名称 | 版本 | 本地绝对路径 | 说明 |
|-------|------|-------------|------|
| {THIRD_PARTY_LOCAL_ROWS} |

### 3.3 内置依赖

**需要下载安装（需特定运行时版本）：**

| 模块名称 | 所需运行时版本 | 安装方式 | 说明 |
|---------|--------------|---------|------|
| {BUILTIN_DOWNLOAD_ROWS} |

**本地已有：**

| 模块名称 | 本地绝对路径 | 说明 |
|---------|-------------|------|
| {BUILTIN_LOCAL_ROWS} |

> 如果创建 instruction 时未提供依赖模块信息，则本章节可省略或标注为"无额外依赖要求"。

---

## 4. 涉及文件清单

### 4.1 核心代码文件

{CORE_FILES_BY_CATEGORY}

### 4.2 接口文件

**外部接口**
- {EXTERNAL_INTERFACE_FILES}

**内部接口**
- {INTERNAL_INTERFACE_FILES}

### 4.3 配置文件

**系统配置**
- {CONFIG_FILES}

**数据定义**
- {DATA_DEFINITION_FILES}

---

## 5. 目录结构说明

```
{DIRECTORY_STRUCTURE}
```

**目录职责说明：**
- {DIRECTORY_RESPONSIBILITIES}

---

## 6. 架构设计原则

### 6.1 模块间依赖关系

{MODULE_DEPENDENCY_RULES}

**依赖流向图：**
```mermaid
graph TD
    {DEPENDENCY_DIAGRAM}
```

### 6.2 数据处理流程

{DATA_PROCESSING_FLOW}

### 6.3 系统交互模式

{SYSTEM_INTERACTION_PATTERN}

---

## 7. 后续开发扩展指导

{EXTENSION_GUIDANCE_SECTIONS}

---

## 8. 常见开发场景

### 8.1 场景一：{SCENARIO_1_NAME}

**触发条件：** {SCENARIO_1_TRIGGER}
**处理流程：** {SCENARIO_1_PROCESS}
**注意事项：** {SCENARIO_1_NOTES}

### 8.2 场景二：{SCENARIO_2_NAME}

**触发条件：** {SCENARIO_2_TRIGGER}
**处理流程：** {SCENARIO_2_PROCESS}
**注意事项：** {SCENARIO_2_NOTES}

### 8.3 场景三：{SCENARIO_3_NAME}

**触发条件：** {SCENARIO_3_TRIGGER}
**处理流程：** {SCENARIO_3_PROCESS}
**注意事项：** {SCENARIO_3_NOTES}

---

## 9. 代码质量要求

### 9.1 命名规范

{NAMING_CONVENTIONS}

### 9.2 注释要求

{COMMENT_REQUIREMENTS}

### 9.3 测试要求

{TEST_REQUIREMENTS}

### 9.4 错误处理

{ERROR_HANDLING_RULES}

---

## 10. 性能与优化建议

### 10.1 性能考虑点

{PERFORMANCE_CONSIDERATIONS}

### 10.2 资源管理

{RESOURCE_MANAGEMENT}

### 10.3 系统优化

{SYSTEM_OPTIMIZATION}

---

## 11. 已知限制与注意事项

### 11.1 技术限制

{TECHNICAL_LIMITATIONS}

### 11.2 设计约束

{DESIGN_CONSTRAINTS}

### 11.3 兼容性要求

{COMPATIBILITY_REQUIREMENTS}

---

## 12. 故障排查指南

### 12.1 常见问题

{COMMON_ISSUES}

### 12.2 调试建议

{DEBUG_SUGGESTIONS}

### 12.3 日志规范

{LOG_STANDARDS}

---

## 附录：相关参考

- **原始需求文档**: [链接到FR文档]
- **实现计划文档**: [链接到Plan文档]  
- **归档总结文档**: [链接到归档文档]
- **相关API文档**: {API_REFERENCES}
- **架构设计文档**: {ARCHITECTURE_REFERENCES}