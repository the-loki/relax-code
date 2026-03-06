# relax-code 项目约定

## 项目目标

- 本项目用于构建一个类似 `codex cli` / `claude code` 的本地 AI coding agent。
- 当前采用的路线是：`可扩展内核 + 最小可用 CLI`。
- 当前工作树已完成实施计划 `Task 1` 到 `Task 9` 的最小骨架与文档收口。

## 新会话必读

- 进入仓库后，先阅读 `AGENTS.md`。
- 再阅读 `docs/plans/2026-03-06-relax-code-agent-design.md`。
- 然后阅读 `docs/plans/2026-03-06-relax-code-agent-implementation.md`。
- 实现具体模块前，按需阅读：
  - `docs/architecture/agent-loop.md`
  - `docs/architecture/tool-system.md`
  - `docs/architecture/session-state.md`
  - `docs/architecture/prompt-and-skills.md`

## 当前共识

- 使用 Rust workspace 组织工程。
- 保持 `agent loop` 稳定，能力通过模块外挂扩展。
- 首版仅做本地 CLI coding agent，不做 Web UI。
- 首版仅做 OpenAI-compatible provider，不同时做多家 provider。
- 首版聚焦：会话、工具、计划、技能加载，不提前实现多 agent 团队与复杂审批系统。

## 当前实现状态

- 已落地 crate：`relax-cli`、`relax-core`、`relax-runtime`、`relax-tools`、`relax-providers`。
- 已完成的最小能力：
  - `relax chat` 与 `relax resume` CLI 入口
  - `agent loop` 纯文本结束分支与工具调用往返分支
  - `shell`、`read_file`、`write_file`、`update_plan` 最小工具实现
  - OpenAI-compatible 响应解析与最小 HTTP 封装
  - `.relax/sessions/<session-id>.json` 会话落盘与恢复
  - 本地 `skills/<name>/SKILL.md` 读取与 `chat --skill <name>` 入口
- 当前仍属“最小骨架”而非完整产品：
  - `chat --skill` 已接线到提示构造入口，但尚未形成完整可观察的 provider 执行链路
  - `update_plan` 目前提供最小文本化计划更新能力，尚未接到持久化计划系统
  - `resume` 当前只验证会话可读回并恢复到内存，不直接恢复完整 chat loop

## 当前验证方式

- 新会话接手前，先在当前工作树执行：`cargo test`
- 目前可用的最小命令入口：
  - `cargo run -p relax-cli -- chat --workspace .`
  - 当仓库下存在对应技能文件（例如 `skills/example/SKILL.md`）时，可运行：`cargo run -p relax-cli -- chat --workspace . --skill example`
  - 当 `.relax/sessions/demo.json` 已存在时，可运行：`cargo run -p relax-cli -- resume --session demo --workspace .`

## CI 构建制品

- 仓库已存在 artifact workflow：`.github/workflows/build-artifacts.yml`
- 自动触发范围仅限：`push` 到 `main`
- 额外保留 `workflow_dispatch` 仅用于手动补跑，不改变“只对 `main` 自动生效”的主规则
- 该 workflow 不属于 Release 流程：
  - 不创建 GitHub Release
  - 不依赖 tag 触发
  - 不上传外部制品仓库
- 当前 Actions artifact 名称：
  - `relax-ubuntu-latest`
  - `relax-windows-latest`
  - `relax-macos-latest`
- 下载方式：
  - 打开 GitHub 仓库的 `Actions`
  - 进入 `Build Artifacts` workflow 的某次运行
  - 在页面底部 `Artifacts` 区域下载对应平台产物
- 手动补跑方式：
  - 打开 GitHub 仓库的 `Actions`
  - 进入 `Build Artifacts` workflow
  - 点击 `Run workflow` 手动触发
- 当前未完成项：
  - GitHub Release
  - 制品签名与公证
  - 安装包制作
  - 交叉编译

## 推荐目录

- `crates/relax-cli/`：CLI 入口、参数解析、交互输出。
- `crates/relax-core/`：`agent loop`、消息模型、执行上下文。
- `crates/relax-tools/`：工具 trait、工具注册表、内置工具实现。
- `crates/relax-providers/`：模型供应商适配层。
- `crates/relax-runtime/`：配置、日志、会话持久化、本地运行态目录。
- `skills/`：项目本地技能目录。
- `tests/`：集成测试与流程测试。
- `.relax/`：本地运行态目录，必须忽略提交。

## 阶段边界

- Stage 0：CLI 骨架、基础配置、最小可运行命令。
- Stage 1：`agent loop`、工具注册、最小 `shell` 工具。
- Stage 2：文件工具、会话持久化、恢复能力。
- Stage 3：`update_plan` / todo 能力。
- Stage 4：技能加载与提示组装。
- Stage 5：subagent 与任务拆分。
- Stage 6+：上下文压缩、后台任务、工作树隔离、多 agent 协作。

## 明确暂不做

- 不先做 Web UI。
- 不先做 MCP 全量协议。
- 不先做复杂权限审批界面。
- 不先做多 provider 并行适配。
- 不先做自动代码评审流水线。

## 实施要求

- 每次实现前，先确认当前任务对应的阶段，不要跨阶段扩展。
- 优先遵循 `docs/plans/2026-03-06-relax-code-agent-implementation.md` 的任务顺序。
- 保持 TDD 倾向：先写失败测试，再写最小实现，再验证通过。
- 避免过度设计，优先可运行、可验证、可续做。
- 运行态、设计决策、架构说明都要落盘，不只保留在会话里。

## 交付要求

- 新增实现后，应同步更新相关架构文档与计划状态。
- 若偏离既定路线，必须先在文档中说明原因与取舍。
- 除非用户明确要求，否则不要提交 git commit。

## 后续重点

- 优先把 `chat` 命令真正接到 provider 调用与 system prompt 输入链路。
- 让 `update_plan` 从文本占位实现演进为可持久化的计划状态。
- 为工具失败、技能入口、会话错误路径补更多行为测试。
- Stage 5 之后的 subagent、上下文压缩、后台任务、工作树隔离仍属于后续范围。
