# Build Artifacts Workflow 设计文档

## 1. 背景

当前仓库已经具备 Rust workspace 和最小可用 CLI，但还没有 GitHub Actions 工作流来为 `main` 分支自动构建并上传可下载制品。

本次需求的目标不是创建 GitHub Release，也不是发布到外部制品仓库，而是在 GitHub Actions 的运行页面中提供可直接下载的构建产物。

## 2. 目标

为仓库新增一个只对 `main` 生效的 GitHub Actions workflow，自动完成以下流程：

- 拉取代码
- 安装 Rust toolchain
- 执行测试
- 构建 `relax` 二进制
- 将二进制按平台打包为 artifact
- 在 Actions 页面提供下载入口

## 3. 非目标

以下内容明确不在本次范围内：

- GitHub Release
- tag 驱动发布
- 上传到对象存储、包仓库或外部制品系统
- PR 触发构建制品
- 交叉编译工具链配置
- 签名、公证、安装包制作

## 4. 方案选择

### 4.1 备选方案

#### 方案 A：单平台构建

只在一个 runner 上构建并上传单份 artifact。

- 优点：最简单
- 缺点：下载可用性差，无法覆盖主要平台

#### 方案 B：多平台矩阵构建

在 `ubuntu-latest`、`windows-latest`、`macos-latest` 上分别构建并上传 artifact。

- 优点：用户可以直接下载对应平台产物，实用性最高
- 缺点：运行时间比单平台更长

#### 方案 C：拆分测试 workflow 与制品 workflow

把测试和制品构建拆成两个独立 workflow。

- 优点：职责更清晰
- 缺点：对当前项目来说偏超前，维护成本更高

### 4.2 最终方案

采用 **方案 B：多平台矩阵构建**。

原因：

- 本项目是 CLI，artifact 的直接可下载价值很高
- 当前没有复杂平台差异，适合使用 GitHub 官方 runner 原生构建
- 仍然只使用一个 workflow 文件，复杂度可控

## 5. workflow 设计

### 5.1 文件位置

新增：

- `.github/workflows/build-artifacts.yml`

当前采用单文件 workflow 承载该流程，避免在当前阶段拆分多个 workflow 文件。

### 5.2 触发条件

```yaml
on:
  push:
    branches:
      - main
  workflow_dispatch:
```

设计说明：

- 主规则只对 `main` 的 `push` 生效
- 额外保留 `workflow_dispatch`，便于手动补跑，不改变主触发策略

### 5.3 Job 结构

仅保留一个 `build` job，使用 matrix：

- `ubuntu-latest`
- `windows-latest`
- `macos-latest`

每个平台执行相同步骤。

### 5.4 执行步骤

每个平台 job 的步骤：

1. `actions/checkout`
2. 安装 stable Rust toolchain
3. 启用 Cargo 缓存
4. `cargo test --quiet`
5. `cargo build --release -p relax-cli`
6. 收集平台对应二进制
7. 打包为 zip
8. `actions/upload-artifact`

### 5.5 制品命名

推荐 artifact 名称：

- `relax-ubuntu-latest`
- `relax-windows-latest`
- `relax-macos-latest`

推荐压缩包名称：

- `relax-ubuntu-latest.zip`
- `relax-windows-latest.zip`
- `relax-macos-latest.zip`

当前阶段不把架构名写死到文件名里，避免与 GitHub runner 实际架构不一致。

### 5.6 制品内容

每个 zip 中只包含最小需要文件：

- Linux/macOS：`relax`
- Windows：`relax.exe`

当前不额外打包 README、LICENSE、配置样例等附加材料。

### 5.7 保留策略

artifact 保留时间建议为：

- `14` 天

这样既便于下载和排查，也不会让存储长期堆积。

### 5.8 当前落地状态

当前 workflow 已落地为：

- 文件路径：`.github/workflows/build-artifacts.yml`
- 触发条件：
  - `push` 到 `main`
  - `workflow_dispatch`
- 当前 artifact 名称：
  - `relax-ubuntu-latest`
  - `relax-windows-latest`
  - `relax-macos-latest`

当前行为说明：

- 每个平台先执行 `cargo test --quiet`
- 再执行 `cargo build --release -p relax-cli`
- 将平台对应二进制压缩为 zip 后上传到 Actions artifact
- 整个流程不涉及 Release 语义

## 6. 实现边界

### 6.1 当前阶段会做

- 新增一个 workflow 文件
- 只在 `main` 上自动构建
- 生成并上传多平台 artifact
- 保证在 CI 中执行现有测试

### 6.2 当前阶段不会做

- 发布 Release
- 自动生成版本号
- 签名和公证
- 交叉编译
- 自动创建 PR 注释
- 构建说明页面

## 7. 验证策略

本次改动属于 CI 配置改动，主要验证方式包括：

- 本地执行 `cargo test --quiet`，确认 workflow 引用的基础构建命令正确
- 检查 workflow YAML 结构是否与仓库当前 Cargo workspace 一致
- 推到 `main` 后，在 GitHub Actions 页面确认：
  - 三个平台 job 正常运行
  - artifact 可下载
  - artifact 名称符合约定

下载说明：

- 进入 GitHub 仓库的 `Actions`
- 打开 `Build Artifacts` workflow 的某次运行记录
- 在运行详情页底部的 `Artifacts` 区域下载对应平台产物

手动触发说明：

- 进入 GitHub 仓库的 `Actions`
- 打开 `Build Artifacts` workflow
- 点击 `Run workflow` 手动触发

## 8. 对新会话的意义

该设计会把“构建产物如何生成、触发范围是什么、为什么不做 Release”固定到仓库文档中，避免后续会话把 artifact workflow 误实现成 release 流程。

## 8.1 当前未完成项

- GitHub Release
- 制品签名与公证
- 安装包制作
- 交叉编译

## 9. 当前结论

本次最合适的实现是：

- 新增 `.github/workflows/build-artifacts.yml`
- 只对 `main` push 自动执行
- 使用多平台矩阵构建
- 在 Actions 页面上传可下载 artifact
- 不涉及 Release 语义
