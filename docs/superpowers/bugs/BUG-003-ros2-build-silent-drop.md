# BUG-003: ROS2 bridge 节点的 `build` 和 `path_sha256` 被静默丢弃

## 元信息

| 字段 | 值 |
|------|-----|
| **编号** | BUG-003 |
| **日期** | 2026-08-04 |
| **分支** | bug/descriptor-ros2-build-silent-drop |
| **严重程度** | minor |
| **分类** | descriptor |
| **状态** | fixed |

## 发现

分析 `resolve_aliases_and_set_defaults` 中的 ros2→Custom 转换（mod.rs:182-209），发现 `build: None` 和 `path_sha256: None` 被硬编码。用户在 ros2 节点上设置的 `build` 和 `path_sha256` 被静默丢弃，且 `kind()` 不拒绝这个组合。

## 复现

```yaml
nodes:
  - id: my_bridge
    ros2:
      topic: /odom
      message_type: nav_msgs/Odometry
      direction: subscribe
    build: "pip install extra-package"    # 静默丢弃
```

**预期行为**：报错提示 `build` 与 `ros2` 不兼容。

**实际行为**：ROS2 bridge 正常运行，但 `build` 命令未执行。

## 根因

- 文件: `libraries/core/src/descriptor/mod.rs:185,187`
- 原因: ros2→Custom 转换中 `build` 和 `path_sha256` 被硬编码为 `None`，未从 node 传播。且 `kind()` 不检查 ros2+source 字段的互斥。

## 修复

| 字段 | 值 |
|------|-----|
| **Commit** | `待提交` |
| **PR** | (待创建) |
| **改动文件** | `libraries/core/src/descriptor/mod.rs` |
| **改动摘要** | 在 `resolve_aliases_and_set_defaults` 中 ros2 节点处理前增加 build/path_sha256 互斥检查，列出冲突字段名 |

## 验证

- [x] 新增回归测试通过: `ros2_rejects_build_and_path_sha256` (2个字段用例)
- [x] 全量测试通过: 279 passed, 0 failed
- [x] Clippy 通过
- [x] 格式化通过

## 关联

- 相关 BUG: BUG-001, BUG-002
- 参考 PR: #2911
