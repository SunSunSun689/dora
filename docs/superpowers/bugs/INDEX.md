# Bug 跟踪索引

## 整体进度

| 分类 | 扫描 | Bug | Fixed | 状态 |
|------|:--:|:--:|:-----:|:----:|
| descriptor | ✅ | 3 | 3 | 完成 |
| routing | ⬜ | 0 | 0 | 待扫描 |
| lifecycle | ✅ | 1 | 0 | 进行中 |
| bridge | ⬜ | 0 | 0 | 待扫描 |
| binding | ⬜ | 0 | 0 | 待扫描 |
| cli | ⬜ | 0 | 0 | 待扫描 |

## 分类统计

| 分类 | Open | Fixed | Total |
|------|------|-------|-------|
| descriptor | 0 | 3 | 3 |
| routing | 0 | 0 | 0 |
| lifecycle | 1 | 0 | 1 |
| bridge | 0 | 0 | 0 |
| binding | 0 | 0 | 0 |
| cli | 0 | 0 | 0 |

## 全部记录

| 编号 | 标题 | 分类 | 严重程度 | 状态 | 日期 | 分支/PR |
|------|------|------|---------|------|------|---------|
| [BUG-001](BUG-001-module-silent-drop-env.md) | 模块节点 `env` 等配置字段静默丢弃 | descriptor | major | fixed | 2026-08-04 | bug/descriptor-silent-field-drops |
| [BUG-002](BUG-002-hub-path-silent-drop.md) | `hub` + `path` 同时设置时 hub 被静默丢弃 | descriptor | major | fixed | 2026-08-04 | bug/descriptor-silent-field-drops |
| [BUG-003](BUG-003-ros2-build-silent-drop.md) | ROS2 bridge 节点的 `build` 和 `path_sha256` 被静默丢弃 | descriptor | minor | fixed | 2026-08-04 | bug/descriptor-silent-field-drops |
| [BUG-004](BUG-004-startup-barrier-no-timeout.md) | 启动屏障无超时，未注册节点永久卡住数据流 | lifecycle | major | open | 2026-08-04 | — |
