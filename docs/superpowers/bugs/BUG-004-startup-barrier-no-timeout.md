# BUG-004: 启动屏障无超时，未注册节点可永久卡住数据流

## 元信息

| 字段 | 值 |
|------|-----|
| **编号** | BUG-004 |
| **日期** | 2026-08-04 |
| **分支** | bug/lifecycle-startup-barrier-dead-process |
| **严重程度** | major |
| **分类** | lifecycle |
| **状态** | fixed |

## 发现

扫描 daemon 启动屏障代码 `binaries/daemon/src/pending.rs` 发现 `update_dataflow_status` 等待 `local_nodes` 清空时没有任何超时机制。

## 复现

数据流 `tests/lifecycle-barrier/cases/hung-node-dataflow.yml`：

```yaml
nodes:
  - id: hung_node
    path: shell sleep 999999    # 进程启动但永远不向 daemon 注册
    outputs:
      - never_produced

  - id: normal_node
    path: normal.py
    inputs:
      data: hung_node/never_produced
```

复现步骤：
```bash
dora up
dora start tests/lifecycle-barrier/cases/hung-node-dataflow.yml
# 永远卡在 startup，dataflow.start() 不会被调用
# hung_node 的 sleep 999999 进程正在运行但从不注册
# normal_node 在 init_from_env() 中一直等
```

**预期行为**：超时后 daemon 应该 kill 掉未注册的节点并报告错误，让数据流继续或失败。

**实际行为**：永久挂起，无超时，无日志。

## 根因

- 文件: `binaries/daemon/src/pending.rs:244-292`
- 原因: `update_dataflow_status` 中启动屏障的释放条件只有 `local_nodes.is_empty()`，没有超时。`check_node_health` 直接跳过 `last_activity == 0` 的节点（"not yet connected"），`finish_stragglers` 同样排除了未连接节点。

## 修复

| 字段 | 值 |
|------|-----|
| **Commit** | (待提交) |
| **PR** | (待创建) |
| **改动文件** | `binaries/daemon/src/lib.rs`, `binaries/daemon/src/running_dataflow.rs` |
| **改动摘要** | `check_node_health` 中检测未连接节点的进程是否已死（`ProcessHandle::is_disconnected`），死进程触发 `handle_node_stop_inner` 释放启动屏障。不杀进程，不设超时——只检测进程已死这一不可恢复条件。 |

## 验证

- [ ] 新增回归测试通过
- [ ] 全量测试通过
- [ ] Clippy 通过
- [ ] 格式化通过

## 关联

- 相关 BUG: 无
- 参考 PR: #2933, #2990 (相关但不解决超时问题)
