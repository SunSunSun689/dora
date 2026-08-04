# Bug 测试工程流程 & 模板

## 流程规则

1. **代码和测试分步进行**：先完成代码改动并验证，再补测试。不能边写测试边修代码。
2. **代码和测试分文件提交**：生产代码改动和测试改动分两次 commit。不能混在同一个 commit 里。
3. **两种测试方式根据场景选择**：
   - **单元测试**：测函数内部逻辑（如边界值、错误分支），写在产品代码文件的 `#[cfg(test)] mod tests` 块里。标准 Rust 惯例，允许。
   - **集成测试**：测"用户写某 YAML 应该报错/通过"这类端到端行为，写在独立的 `tests/` 目录 + YAML 数据文件，通过 `dora validate` 或 `cargo test --test <name>` 跑。
   - **禁止**：把集成测试写进产品代码文件的 `#[cfg(test)]` 里（像这次最开始的错误做法）。
4. **每个 bug 独立记录**：使用下方模板，保存为 `BUG-XXX-<slug>.md`
5. **每个 bug 独立分支**：`bug/<category>-<short-slug>`。同类 bug 可放同一分支。
6. **TDD**：先写失败测试 → 修复 → 改进
7. **修复后必须验证**：回归测试 + 全量测试 + clippy + fmt
8. **本地保存**：`docs/superpowers/bugs/` 在 `.gitignore` 中，不推送 GitHub

---

## 记录模板

## 元信息

| 字段 | 值 |
|------|-----|
| **编号** | BUG-000 |
| **日期** | YYYY-MM-DD |
| **分支** | bug/<category>-<slug> |
| **严重程度** | critical / major / minor |
| **分类** | descriptor / routing / lifecycle / bridge / binding / cli |
| **状态** | open / in-progress / fixed / wont-fix |

## 发现

（如何发现这个 bug：测试、手工验证、代码审查等）

## 复现

（最小复现步骤，包括输入、环境、命令）

```yaml
# 示例数据流 / 输入
```

**预期行为**：

**实际行为**：

## 根因

（定位过程，最终根因在哪个文件、哪个函数、哪段逻辑）

- 文件: `path/to/file.rs:123`
- 原因:

## 修复

| 字段 | 值 |
|------|-----|
| **Commit** | `xxxxxxxxx` |
| **PR** | #xxxx |
| **改动文件** | `path/to/file.rs` |
| **改动摘要** | |

## 验证

- [ ] 新增回归测试通过: `cargo test -p <crate> <test_name>`
- [ ] 全量测试通过: `cargo test --all --exclude ...`
- [ ] Clippy 通过: `cargo clippy --all --exclude ... -- -D warnings`
- [ ] 格式化通过: `cargo fmt --all -- --check`
- [ ] /review 无新问题
- [ ] /simplify 无改进项

## 关联

- 相关 BUG: 
- 参考 commit: 
- 参考文档: 
