# relax-code 项目约定

## 项目目标

- 本项目用于构建一个类似 `codex cli` / `claude code` 的本地 AI coding agent。
- 当前采用的路线是：`可扩展内核 + 最小可用 CLI`。
- 当前阶段以文档固化与实施计划为主，后续按阶段推进实现。

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
