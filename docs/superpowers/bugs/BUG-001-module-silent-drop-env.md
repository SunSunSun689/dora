# BUG-001: 模块节点 `env` 等配置字段静默丢弃

## 元信息

| 字段 | 值 |
|------|-----|
| **编号** | BUG-001 |
| **日期** | 2026-08-04 |
| **分支** | bug/descriptor-module-silent-drop-env |
| **严重程度** | major |
| **分类** | descriptor |
| **状态** | fixed |

## 发现

扫描 descriptor 模块，发现 #2911 (`validate_module_node_source_fields`) 只检查了 source/build 类字段（`path`、`git`、`operator` 等 12 个），但没有检查配置类字段。用户在模块节点上设置 `env`、`args`、`outputs` 等字段会被静默丢弃，没有任何错误提示。

## 复现

```yaml
nodes:
  - id: m
    module: my_module.yml
    env:
      MY_VAR: hello        # 静默丢弃，内部节点看不到这个 env
    args: "--debug"         # 静默丢弃
    outputs:                # 静默丢弃（模块输出来自 module header）
      - fake_output
```

**预期行为**：报错提示这些字段与 `module` 不兼容。

**实际行为**：模块正常展开，`env`/`args`/`outputs` 被静默丢弃。

## 根因

- 文件: `libraries/core/src/descriptor/expand.rs:638-667`
- 原因: `validate_module_node_source_fields` 只检查了 12 个 source/build 字段，遗漏了配置类字段：
  - `env` — 应该像 `deploy` 一样传播但没传播
  - `args` — 无意义，模块节点不启动进程
  - `outputs` — 模块输出来自 `module.outputs` header
  - `output_types`、`output_framing`、`input_types`、`output_metadata`
  - `pattern` — 无意义
  - `send_stdout_as`、`send_logs_as`、`min_log_level`、`max_log_size`、`max_rotated_files`
  - `restart_policy`、`max_restarts`、`restart_delay`、`max_restart_delay`、`restart_window`
  - `health_check_timeout`、`finish_grace_secs`
  - `shared_memory_pool_size`、`cpu_affinity`

## 修复

| 字段 | 值 |
|------|-----|
| **Commit** | `待提交` |
| **PR** | (待创建) |
| **改动文件** | `libraries/core/src/descriptor/expand.rs`, `libraries/core/src/descriptor/mod.rs` |
| **改动摘要** | 新增 `validate_module_node_fields` 函数，用白名单方式校验模块节点：只允许 `module`/`inputs`/`params`/`deploy`/`id`/`name`/`description`，其余全部拒绝并列出冲突字段名 |

## 验证

- [x] 新增回归测试通过: `reject_unsupported_fields_on_module_node` (12个字段用例)
- [x] 全量测试通过: 279 passed, 0 failed
- [x] Clippy 通过: `cargo clippy -p dora-core --features zenoh -- -D warnings`
- [x] 格式化通过: `cargo fmt --all -- --check`

## 关联

- 相关 BUG: 无
- 参考 PR: #2911 (同类型修复)
- 参考 commit: `a6588cf20 fix(core): reject module nodes with source fields`
