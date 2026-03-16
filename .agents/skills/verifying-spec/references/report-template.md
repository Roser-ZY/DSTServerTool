# {PROJECT_NAME} 规格验证报告

> **验证日期**: {DATE}
> **验证范围**: {SCOPE_DESCRIPTION}
> **验证触发**: {TRIGGER}（brainstorm 后 / 文档完成后 / 手动触发）
> **文档根目录**: {SPEC_ROOT}

---

## 验证概要

| 指标 | 结果 |
|------|------|
| **总规则数** | {TOTAL_RULES} |
| **通过 (PASS)** | {PASS_COUNT} |
| **失败 (FAIL)** | {FAIL_COUNT} |
| **警告 (WARN)** | {WARN_COUNT} |
| **跳过 (SKIP)** | {SKIP_COUNT} |
| **验证结论** | {VERDICT}: PASS / PASS WITH WARNINGS / FAIL |

---

## 验证输入

### 原始需求来源

| 来源 | 路径/描述 |
|------|-----------|
| {SOURCE_TYPE_1} | {SOURCE_PATH_1} |
| {SOURCE_TYPE_2} | {SOURCE_PATH_2} |

### 待验证文档

| 文档类型 | 路径 | 状态 |
|----------|------|------|
| FR 文档 | {FR_PATH} | 已读取 / 未找到 |
| FI 文档 | {FI_PATH} | 已读取 / 未找到 / 不适用 |
| Design 文档 | {DESIGN_PATH} | 已读取 / 未找到 |
| Plan 文档 | {PLAN_PATH} | 已读取 / 未找到 |
| Brainstorm 文档 | {BRAINSTORM_PATH} | 已读取 / 不适用 |
| Instruction 文档 | {INSTRUCTION_PATH} | 已读取 / 不适用 |

---

## Layer 1: 原始需求一致性

### L1-001 功能点完整覆盖 — {PASS/FAIL/WARN}

**原始需求功能点**: {N} 项
**FR 文档覆盖**: {M} 项

{如 PASS}:
> 所有原始需求功能点均已在 FR 文档中覆盖。

{如 FAIL/WARN，列出详细清单}:

| 状态 | 原始需求 | FR 编号 | 说明 |
|------|----------|---------|------|
| ✅ | {需求描述} | FR-0001 | 完全覆盖 |
| ❌ | {需求描述} | — | **遗漏**: 原始需求中明确要求，FR 中未体现 |
| ⚠️ | — | FR-0003 | **新增**: FR 中存在但原始需求未提及，来源: {来源} |

### L1-002 非功能需求保留 — {PASS/FAIL/WARN}

{验证结果详情}

### L1-003 范围边界一致 — {PASS/FAIL/WARN}

{验证结果详情}

### L1-004 术语一致性 — {PASS/WARN}

{验证结果详情}

---

## Layer 2: Brainstorm/Explore 一致性

{如不适用}:
> SKIP: 本次 SDD 流程未包含 brainstorm/explore 阶段。

{如适用}:

### L2-001 Brainstorm 结论完整落地 — {PASS/FAIL}

**Brainstorm 确认结论**: {N} 项

| 状态 | Brainstorm 结论 | 对应文档位置 |
|------|-----------------|-------------|
| ✅ | {结论描述} | FR-0001 / design §2.3 |
| ❌ | {结论描述} | **丢失**: 未在任何后续文档中体现 |

### L2-002 功能边界与 Brainstorm 共识一致 — {PASS/FAIL}

{验证结果详情}

### L2-003 方案选择一致 — {PASS/FAIL/WARN}

{验证结果详情}

### L2-004 Explore 发现事项反映 — {PASS/WARN}

{验证结果详情}

---

## Layer 3: 文档间交叉一致性

### L3-001 FR -> Design 映射完整性 — {PASS/FAIL}

| FR 编号 | DM 编号 | 状态 |
|---------|---------|------|
| FR-0001 | DM-FR-0001 | ✅ 已映射 |
| FR-0002 | — | ❌ 设计缺失 |
| — | DM-FR-0005 | ❌ 孤立设计，无对应 FR |

### L3-002 FR -> Plan 任务覆盖 — {PASS/FAIL/WARN}

{验证结果详情}

### L3-003 Design -> Plan 技术一致性 — {PASS/FAIL/WARN}

{验证结果详情}

### L3-004 FR <-> FI 对应关系 — {PASS/FAIL/SKIP}

{验证结果详情}

### L3-005 接口定义一致 — {PASS/FAIL}

{验证结果详情}

---

## Layer 4: Instruction 合规性

{如不适用}:
> SKIP: 本次 SDD 流程未涉及 instruction 文档。

{如适用}:

### L4-001 架构层级规范遵循 — {PASS/FAIL/WARN}

{验证结果详情}

### L4-002 模块扩展规范遵循 — {PASS/FAIL}

{验证结果详情}

### L4-003 命名与目录规范遵循 — {PASS/FAIL/WARN}

{验证结果详情}

### L4-004 自治 Instruction 输入要求满足 — {PASS/FAIL/SKIP}

{验证结果详情}

### L4-005 依赖模块规范遵循 — {PASS/FAIL/WARN}

{验证结果详情}

---

## 通用规则

### G-001 编号唯一性 — {PASS/FAIL}

{验证结果详情}

### G-002 编号连续性 — {PASS/WARN}

{验证结果详情}

### G-003 交叉引用有效性 — {PASS/FAIL}

{验证结果详情}

### G-004 优先级一致性 — {PASS/FAIL}

{验证结果详情}

---

## 问题汇总

### 必须修复 (FAIL)

| # | 规则 | 问题描述 | 影响文档 | 建议修复 |
|---|------|----------|----------|----------|
| 1 | {规则编号} | {问题描述} | {文件路径} | {修复建议} |

### 建议关注 (WARN)

| # | 规则 | 问题描述 | 影响文档 | 建议 |
|---|------|----------|----------|------|
| 1 | {规则编号} | {问题描述} | {文件路径} | {处理建议} |

---

## 后续动作

{根据验证结论自动生成}

**如 FAIL**:
- [ ] 修复上述 FAIL 项后，重新运行 verifying-spec
- [ ] 修复涉及的文档: {文件列表}

**如 PASS WITH WARNINGS**:
- [ ] 评估 WARN 项是否需要处理
- [ ] 可继续进入下一 SDD 阶段

**如 PASS**:
- [ ] 文档验证通过，可进入下一阶段

---

> 报告由 `verifying-spec` 技能自动生成。
> 验证规则版本: v1.0
