# Bug 跟踪索引

## 整体进度

| 分类 | 扫描 | Bug | Fixed | 状态 |
|------|:--:|:--:|:-----:|:----:|
| descriptor | ✅ | 3 | 3 | 完成 |
| routing | ⬜ | 0 | 0 | 待扫描 |
| lifecycle | ✅ | 1 | 1 | 进行中（routing 扫描发现 FINDING P/K+M/E 待处理） |
| bridge | ⬜ | 0 | 0 | 待扫描 |
| binding | ⬜ | 0 | 0 | 待扫描 |
| cli | ⬜ | 0 | 0 | 待扫描 |

## 分类统计

| 分类 | Open | Fixed | Total |
|------|------|-------|-------|
| descriptor | 0 | 3 | 3 |
| routing | 3 (P/K+M/E) | 0 | 3 |
| lifecycle | 0 | 1 | 1 |
| bridge | 0 | 0 | 0 |
| binding | 0 | 0 | 0 |
| cli | 0 | 0 | 0 |

## 全部记录

| 编号 | 标题 | 分类 | 严重程度 | 状态 | 日期 | 分支/PR |
|------|------|------|---------|------|------|---------|
| [BUG-001](BUG-001-module-silent-drop-env.md) | 模块节点 `env` 等配置字段静默丢弃 | descriptor | major | fixed | 2026-08-04 | bug/descriptor-silent-field-drops |
| [BUG-002](BUG-002-hub-path-silent-drop.md) | `hub` + `path` 同时设置时 hub 被静默丢弃 | descriptor | major | fixed | 2026-08-04 | bug/descriptor-silent-field-drops |
| [BUG-003](BUG-003-ros2-build-silent-drop.md) | ROS2 bridge 节点的 `build` 和 `path_sha256` 被静默丢弃 | descriptor | minor | fixed | 2026-08-04 | bug/descriptor-silent-field-drops |
| [BUG-004](BUG-004-startup-barrier-no-timeout.md) | 启动屏障无超时，未注册节点永久卡住数据流 | lifecycle | major | fixed | 2026-08-04 | bug/lifecycle-startup-barrier-dead-process |

## Routing 扫描发现（待分类）

| 编号 | 问题 | 严重度 | 位置 |
|------|------|:---:|------|
| FINDING-P | 跨daemon通道满时OutputClosed被丢弃，remote消费者挂死 | medium | `lib.rs:4892-4948` |
| FINDING-K+M | 数据流销毁的节点连接无断开信号，数据静默丢掉 | medium | `node_communication/mod.rs` |
| FINDING-E | 跨daemon输出时finish-straggler watchdog被全局禁用 | medium | `running_dataflow.rs:1032` |
