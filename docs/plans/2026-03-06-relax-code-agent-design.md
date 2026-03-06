# Relax Code Agent 设计文档

## 1. 背景与目标

`relax-code` 的目标是构建一个类似 `codex cli` / `claude code` 的本地 AI coding agent。该 agent 需要在终端中运行，支持多轮对话、工具调用、会话续做和逐步扩展能力。

本项目参考了 `shareAI-lab/learn-claude-code` 的核心思想：保持 `agent loop` 稳定，通过阶段化方式逐步叠加工具、计划、技能、任务与协作能力。但本项目不是教学复刻，而是一个面向持续演进的 Rust 工程。

## 2. 核心设计结论

### 2.1 路线选择

采用 `可扩展内核 + 最小可用 CLI` 路线。

- 不先做一次性脚本式 MVP，因为后续加入 `todo`、`skills`、`subagent` 时会造成大面积返工。
- 也不先做过度抽象的完整平台，因为空仓起步时最容易在权限系统、事件总线、复杂任务调度上投入过多时间。
- 最合理的路径是：先确定稳定骨架，再按阶段补足能力。

### 2.2 技术方向

- 语言：Rust
- 工程形态：Rust workspace
- 运行形态：本地 CLI
- 首版 provider：OpenAI-compatible API
- 首版 provider 范围：只提供单一 OpenAI-compatible 适配器，不同时实现多 provider
- 架构原则：`agent loop` 固定，能力模块外挂

## 3. 非目标

以下能力明确不属于首版范围：

- Web UI
- MCP 全量生态接入
- 多 provider 同时完整支持
- 复杂权限审批界面
- 多 agent 团队协作
- 工作树隔离
- 自动化代码评审流水线

这些能力将在基础骨架稳定后再按阶段引入。

## 4. 仓库结构设计

建议仓库逐步演化为以下结构：

```text
relax-code/
├─ AGENTS.md
├─ docs/
│  ├─ plans/
│  └─ architecture/
├─ crates/
│  ├─ relax-cli/
│  ├─ relax-core/
│  ├─ relax-tools/
│  ├─ relax-providers/
│  └─ relax-runtime/
├─ skills/
├─ tests/
└─ .relax/
```

各目录职责如下：

- `crates/relax-cli/`：命令行入口、参数解析、交互模式、输出渲染。
- `crates/relax-core/`：`agent loop`、消息模型、会话上下文、执行流程。
- `crates/relax-tools/`：工具抽象、注册表、内置工具实现。
- `crates/relax-providers/`：模型供应商适配。
- `crates/relax-runtime/`：配置加载、日志、会话持久化、运行态目录。
- `skills/`：本地技能知识目录。
- `tests/`：集成测试与流程测试。
- `.relax/`：运行时生成的本地状态目录。

## 5. 模块设计

### 5.1 `relax-cli`

职责：

- 处理子命令与参数，例如 `chat`、`resume`、`sessions`。
- 渲染用户可见输出。
- 组装运行参数后调用 `relax-core`。

限制：

- 不承载模型逻辑。
- 不直接处理工具执行细节。

### 5.2 `relax-core`

职责：

- 实现稳定的 `agent loop`。
- 定义核心模型，如 `Message`、`ToolCall`、`ToolResult`、`SessionState`、`TurnContext`。
- 协调 provider、tool registry 和 runtime。

这是整个项目的稳定中枢。

### 5.3 `relax-tools`

职责：

- 提供统一的工具 trait。
- 提供工具 schema 导出能力。
- 提供工具注册表与调用分发。

首版工具范围：

- `shell`
- `read_file`
- `write_file`
- `update_plan`

### 5.4 `relax-providers`

职责：

- 抽象模型供应商差异。
- 向上暴露统一聊天接口。
- 将 provider 响应转为统一的文本块与工具调用块。

首版只实现 OpenAI-compatible provider。

### 5.5 `relax-runtime`

职责：

- 加载配置。
- 管理 `.relax/` 路径。
- 管理会话存储与恢复。
- 提供日志与基础运行支持。

后续权限策略、任务持久化也优先进入本模块。

## 6. 核心数据流

```text
CLI 输入
  -> runtime 加载配置与会话
  -> core 组装系统提示、历史消息、技能内容与用户输入
  -> providers 调用模型
  -> 如果返回文本，则结束本轮
  -> 如果返回工具调用，则 tools 执行工具
  -> 工具结果回填到消息流
  -> core 继续循环，直到模型停止调用工具
  -> runtime 持久化 SessionState
```

设计重点：

- `agent loop` 不感知工具具体实现。
- `agent loop` 不感知 provider 的具体 HTTP 差异。
- 每轮对话结束后必须可恢复。

## 7. 会话续做策略

为了让新会话能够继续推进，必须同时固化三类状态：

### 7.1 代码状态

通过项目级 `AGENTS.md` 和稳定的目录结构表达。

### 7.2 运行状态

通过 `.relax/` 目录表达，例如：

```text
.relax/
├─ sessions/
├─ tasks/
├─ cache/
└─ skills-index.json
```

`SessionState` 至少保存：

- 会话 ID
- 工作目录
- provider / model
- 消息历史
- 当前计划
- 已启用技能
- 最近摘要

### 7.3 决策状态

通过设计文档、实施计划和架构文档表达。新会话不仅要知道“当前代码是什么”，还要知道“为什么这样设计”。

## 8. 技能与提示组装

系统提示不应写成一个巨大且不可维护的单体字符串，而应由多个来源组合：

- 基础系统提示
- 仓库级说明
- 当前任务上下文
- 技能注入内容
- 运行时安全与边界约束

技能内容优先来自本地 `skills/` 目录，后续可扩展到外部来源。

## 9. 分阶段路线图

### Stage 0

- 建立 Rust workspace
- 完成 CLI 骨架
- 加入基础配置加载

### Stage 1

- 实现 `agent loop`
- 实现工具注册机制
- 接入最小 `shell` 工具

### Stage 2

- 加入文件读写工具
- 实现会话持久化与恢复

### Stage 3

- 实现 `update_plan` / todo 能力
- 让 agent 能显式管理中间步骤

### Stage 4

- 实现技能加载
- 实现提示词拼装

### Stage 5

- 引入 subagent 与任务拆分

### Stage 6+

- 上下文压缩
- 后台任务
- 多 agent 协作
- 工作树隔离

## 10. 首版技术选型

- CLI：`clap`
- 异步运行时：`tokio`
- HTTP 客户端：`reqwest`
- 序列化：`serde`、`serde_json`
- 配置：`toml` + 环境变量
- 错误处理：`anyhow`、`thiserror`
- 日志：`tracing`、`tracing-subscriber`

## 11. 测试策略

单元测试优先覆盖：

- provider 响应解析
- 工具注册与分发
- `agent loop` 状态流转
- 会话持久化读写

集成测试优先覆盖：

- CLI 帮助与参数解析
- 一次完整的工具调用闭环
- 会话保存与恢复

首版测试优先使用 mock provider，避免强依赖真实模型 API。

## 12. 成功标准

首版完成时，应该满足以下条件：

- 能在终端中启动 CLI 并发起多轮对话。
- 能触发最小工具调用并把结果回填给模型。
- 能将会话落盘并恢复。
- 能通过文档和仓库约定，让全新会话在 5 分钟内理解当前项目状态。

## 13. 当前结论

本项目不是从“大而全平台”起步，而是从“稳定骨架 + 阶段演进”起步。所有后续实现都应围绕这一结论展开，避免超前设计和范围失控。

## 14. 当前落地状态（2026-03-06）

截至当前工作树状态，实施计划中的 `Task 1` 到 `Task 8` 已完成最小落地：

- `relax-cli`
  - 已提供 `chat` 与 `resume` 子命令
  - `chat --skill <name>` 已接入本地技能加载入口
- `relax-core`
  - 已提供最小 `agent loop`
  - 已支持两条基础分支：纯文本结束、工具调用往返
- `relax-runtime`
  - 已提供 `RuntimePaths`、`Config`、`SessionStore`、`SkillLoader`
  - 已支持 `.relax/sessions/<session-id>.json` 会话读写
- `relax-tools`
  - 已提供 `Tool` trait、`ToolRegistry`
  - 已落地 `shell`、`read_file`、`write_file`、`update_plan` 最小工具
- `relax-providers`
  - 已提供单一 OpenAI-compatible provider 抽象、响应解析器和最小 HTTP 封装

当前也有几项明确的“已接线但未完全闭环”能力：

- `chat --skill` 目前能读取技能并构造 system prompt，但尚未接到真实 provider 执行链路
- `update_plan` 目前只提供最小文本化计划更新能力，尚未接到持久化计划系统
- `resume` 目前只验证会话能够被读回并恢复到内存，不直接恢复完整 chat 会话执行

下一阶段最值得优先推进的方向是：

1. 让 `chat` 命令真正把 prompt、skill、provider 与 `agent loop` 串起来
2. 补充工具错误、会话错误和技能 CLI 入口的行为测试
3. 让计划能力从“文本占位”收敛到真实状态更新接口
