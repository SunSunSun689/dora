# BUG-002: `hub` + `path` 同时设置时 hub 被静默丢弃

## 元信息

| 字段 | 值 |
|------|-----|
| **编号** | BUG-002 |
| **日期** | 2026-08-04 |
| **分支** | bug/descriptor-hub-path-silent-drop |
| **严重程度** | major |
| **分类** | descriptor |
| **状态** | fixed |

## 发现

分析 `kind()` 函数发现：当 `hub` 和 `path` 同时设置时，`kind()` 只在 `hub` 单独出现时（无 `path`）报错。如果两者都设置，匹配到 `Standard(path)`，`hub` 被静默丢弃。

虽然 `dora build` 路径中 `resolve_hub_nodes` 会拒绝这个组合，但 `dora start` 跳过 hub 解析，用户手工写的描述符中 `hub`+`path` 会被静默接受并以 `path` 运行。

## 复现

```yaml
nodes:
  - id: bad_node
    path: local_node.py
    hub: some-package@^1.0    # 静默丢弃，只运行 local_node.py
```

**预期行为**：报错提示 `hub` 和 `path` 互斥。

**实际行为**：静默以 `path` 运行，`hub` 被忽略。

## 根因

- 文件: `libraries/core/src/descriptor/mod.rs:514-524`
- 原因: `kind()` 的 guard 条件是 `self.hub.is_some() && self.path.is_none()`，只覆盖了 hub 单独出现的情况。当两者都设置时，guard 不触发，进入 match 后匹配 `Standard(path)`。
- `resolve_hub_nodes` (hub.rs:447-462) 有完整的互斥检查，但只在 `dora build` 路径运行。

## 修复

| 字段 | 值 |
|------|-----|
| **Commit** | `待提交` |
| **PR** | (待创建) |
| **改动文件** | `libraries/core/src/descriptor/mod.rs` |
| **改动摘要** | 重写 `kind()` 中 hub guard：先检查 hub 是否与 path/operators/custom/operator/ros2/module 冲突，列出所有冲突字段名，再处理 hub-only 的未解析情况 |

## 验证

- [x] 新增回归测试通过: `hub_is_mutually_exclusive_with_other_source_fields` (5个字段用例)
- [x] 全量测试通过: 279 passed, 0 failed
- [x] Clippy 通过
- [x] 格式化通过

## 关联

- 相关 BUG: BUG-001
- 参考 PR: #2911
