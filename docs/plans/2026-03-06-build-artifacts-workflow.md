# Build Artifacts Workflow Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 为 `main` 分支新增一个 GitHub Actions workflow，自动测试、构建并上传 `relax` 的多平台 artifact，且不创建 Release。

**Architecture:** 使用单一 workflow 文件 `.github/workflows/build-artifacts.yml`，在 `push` 到 `main` 和手动 `workflow_dispatch` 时触发。workflow 使用 `ubuntu-latest`、`windows-latest`、`macos-latest` 矩阵执行测试与 release 构建，并将各平台二进制打包后上传到 GitHub Actions artifact。

**Tech Stack:** GitHub Actions YAML、Rust stable toolchain、Cargo、`actions/checkout`、`actions/upload-artifact`。

---

### Task 1: 新增 workflow 骨架与触发条件

**Files:**
- Create: `.github/workflows/build-artifacts.yml`
- Modify: `docs/plans/2026-03-06-build-artifacts-workflow-design.md`

**Step 1: 先验证文件不存在**

Run: `powershell -Command "Test-Path .github/workflows/build-artifacts.yml"`
Expected: `False`

**Step 2: 新增最小 workflow 骨架**

写入：

- `name: Build Artifacts`
- `on.push.branches: [main]`
- `on.workflow_dispatch`
- 单一 `build` job
- `matrix.os` 包含 `ubuntu-latest`、`windows-latest`、`macos-latest`

**Step 3: 自查 YAML 结构与设计文档一致**

对照 `docs/plans/2026-03-06-build-artifacts-workflow-design.md`，确认触发条件和矩阵平台一致。

**Step 4: 记录变更范围**

在设计文档中补充“将由单文件 workflow 承载”这一最终落地形式，如已有则只核对一致性。

**Step 5: Commit**

```bash
git add .github/workflows/build-artifacts.yml docs/plans/2026-03-06-build-artifacts-workflow-design.md
git commit -m "ci: add artifact workflow skeleton"
```

### Task 2: 加入 Rust 安装、缓存、测试与构建步骤

**Files:**
- Modify: `.github/workflows/build-artifacts.yml`

**Step 1: 先确认当前本地构建命令有效**

Run: `cargo test --quiet`
Expected: PASS

**Step 2: 写入 workflow 执行步骤**

为每个平台增加：

- `actions/checkout`
- Rust stable toolchain 安装
- Cargo 缓存
- `cargo test --quiet`
- `cargo build --release -p relax-cli`

**Step 3: 自查二进制路径**

确认 workflow 使用的构建命令会产出：

- `target/release/relax`
- `target/release/relax.exe`

**Step 4: 本地再次验证基础命令**

Run: `cargo build --release -p relax-cli`
Expected: PASS

**Step 5: Commit**

```bash
git add .github/workflows/build-artifacts.yml
git commit -m "ci: add test and build steps"
```

### Task 3: 打包并上传多平台 artifact

**Files:**
- Modify: `.github/workflows/build-artifacts.yml`

**Step 1: 先明确平台产物命名**

在 workflow 中约定 artifact 名：

- `relax-ubuntu-latest`
- `relax-windows-latest`
- `relax-macos-latest`

压缩包名与 artifact 名保持一致。

**Step 2: 写最小打包逻辑**

根据 runner 平台分别将：

- `target/release/relax`
- `target/release/relax.exe`

压缩为 zip 文件。

**Step 3: 写上传步骤**

使用 `actions/upload-artifact` 上传每个平台 zip，设置 `retention-days: 14`。

**Step 4: 自查是否引入 Release 语义**

确认 workflow 中不存在：

- `gh release`
- `actions/create-release`
- tag 触发器

**Step 5: Commit**

```bash
git add .github/workflows/build-artifacts.yml
git commit -m "ci: upload platform artifacts"
```

### Task 4: 补齐文档与使用说明

**Files:**
- Modify: `AGENTS.md`
- Modify: `docs/plans/2026-03-06-build-artifacts-workflow-design.md`

**Step 1: 更新仓库级说明**

在 `AGENTS.md` 中补充：

- 已存在 artifact workflow
- 只对 `main` push 自动触发
- 不属于 Release 流程

**Step 2: 更新设计文档的落地状态**

在设计文档中写明：

- workflow 文件路径
- 当前 artifact 名称
- 当前触发条件

**Step 3: 补充操作说明**

写明用户如何在 GitHub Actions 页面下载 artifact。

**Step 4: 自查是否和实际 workflow 一致**

逐项核对文档与 `.github/workflows/build-artifacts.yml` 一致。

**Step 5: Commit**

```bash
git add AGENTS.md docs/plans/2026-03-06-build-artifacts-workflow-design.md
git commit -m "docs: describe artifact workflow"
```

### Task 5: 最终验证与交付

**Files:**
- Modify: `.github/workflows/build-artifacts.yml`
- Modify: `AGENTS.md`
- Modify: `docs/plans/2026-03-06-build-artifacts-workflow-design.md`

**Step 1: 运行最终本地验证**

Run: `cargo test --quiet`
Expected: PASS

**Step 2: 检查 workflow 关键点**

人工核对：

- 触发条件只包含 `main` push 和 `workflow_dispatch`
- 存在三平台 matrix
- 存在测试、构建、打包、上传步骤
- 不包含 Release 相关动作

**Step 3: 整理交付说明**

说明：

- 推送到 `main` 后自动生成 artifact
- 手动触发方式
- artifact 在 Actions 页面下载

**Step 4: 记录未完成项**

明确后续仍未做：

- Release
- 签名
- 安装包
- 交叉编译

**Step 5: Commit**

```bash
git add .github/workflows/build-artifacts.yml AGENTS.md docs/plans/2026-03-06-build-artifacts-workflow-design.md
git commit -m "ci: finalize artifact workflow"
```
