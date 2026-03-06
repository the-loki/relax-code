# Relax Code Agent Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 构建一个基于 Rust workspace 的本地 CLI coding agent，首版支持对话、工具调用、会话持久化与技能加载。

**Architecture:** 采用 `relax-cli`、`relax-core`、`relax-tools`、`relax-providers`、`relax-runtime` 五层结构，保持 `agent loop` 稳定，通过 provider 与 tool registry 外挂能力。首版只实现 OpenAI-compatible provider，并通过 `.relax/` 目录持久化会话与运行状态。

**Tech Stack:** Rust、Cargo workspace、clap、tokio、reqwest、serde、serde_json、toml、anyhow、thiserror、tracing。

---

### Task 1: 建立 Rust workspace 与最小 CLI

**Files:**
- Create: `Cargo.toml`
- Create: `crates/relax-cli/Cargo.toml`
- Create: `crates/relax-cli/src/main.rs`
- Create: `crates/relax-cli/tests/help_command.rs`
- Modify: `.gitignore`

**Step 1: 写失败测试**

在 `crates/relax-cli/tests/help_command.rs` 中写一个最小集成测试，断言 `relax --help` 输出中包含 `chat` 子命令。

```rust
use assert_cmd::Command;

#[test]
fn help_command_prints_chat_subcommand() {
    let mut cmd = Command::cargo_bin("relax").unwrap();
    cmd.arg("--help");
    cmd.assert().success().stdout(predicates::str::contains("chat"));
}
```

**Step 2: 运行测试确认失败**

Run: `cargo test -p relax-cli help_command_prints_chat_subcommand -- --exact`
Expected: FAIL，因为 workspace 和二进制还不存在。

**Step 3: 写最小实现**

创建 workspace 根 `Cargo.toml`，声明 `crates/relax-cli` 成员；在 `crates/relax-cli/src/main.rs` 中用 `clap` 提供最小 `chat` 子命令。

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "relax")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Chat,
}

fn main() {
    let _cli = Cli::parse();
}
```

**Step 4: 运行测试确认通过**

Run: `cargo test -p relax-cli help_command_prints_chat_subcommand -- --exact`
Expected: PASS。

**Step 5: 记录结果**

更新相关文档中的阶段状态，不执行 git commit，除非用户明确要求。

### Task 2: 建立运行时配置与 `.relax/` 目录约定

**Files:**
- Create: `crates/relax-runtime/Cargo.toml`
- Create: `crates/relax-runtime/src/lib.rs`
- Create: `crates/relax-runtime/src/config.rs`
- Create: `crates/relax-runtime/src/paths.rs`
- Create: `crates/relax-runtime/tests/config_defaults.rs`
- Modify: `docs/architecture/session-state.md`

**Step 1: 写失败测试**

为默认运行态路径和默认配置加载行为写测试。

```rust
#[test]
fn runtime_paths_default_to_dot_relax() {
    let paths = RuntimePaths::from_workspace("/tmp/project");
    assert!(paths.root.ends_with(".relax"));
}
```

**Step 2: 运行测试确认失败**

Run: `cargo test -p relax-runtime runtime_paths_default_to_dot_relax -- --exact`
Expected: FAIL，因为 `RuntimePaths` 尚未实现。

**Step 3: 写最小实现**

实现：

- `RuntimePaths`
- `Config` 与默认值
- 从环境变量与 `relax.toml` 加载配置的最小逻辑

**Step 4: 运行测试确认通过**

Run: `cargo test -p relax-runtime runtime_paths_default_to_dot_relax -- --exact`
Expected: PASS。

**Step 5: 补充文档**

把路径约定同步到 `docs/architecture/session-state.md`。

### Task 3: 定义核心模型与 `agent loop` 外壳

**Files:**
- Create: `crates/relax-core/Cargo.toml`
- Create: `crates/relax-core/src/lib.rs`
- Create: `crates/relax-core/src/message.rs`
- Create: `crates/relax-core/src/session.rs`
- Create: `crates/relax-core/src/agent_loop.rs`
- Create: `crates/relax-core/tests/agent_loop_stops_on_text.rs`
- Modify: `docs/architecture/agent-loop.md`

**Step 1: 写失败测试**

写一个测试，断言当 provider 返回纯文本响应时，`agent loop` 在单轮内结束。

```rust
#[tokio::test]
async fn agent_loop_stops_when_provider_returns_text() {
    let provider = FakeProvider::text("done");
    let result = run_agent_loop(provider).await.unwrap();
    assert_eq!(result.final_text(), "done");
}
```

**Step 2: 运行测试确认失败**

Run: `cargo test -p relax-core agent_loop_stops_when_provider_returns_text -- --exact`
Expected: FAIL，因为 `run_agent_loop` 与 fake provider 还不存在。

**Step 3: 写最小实现**

实现：

- `Message`
- `AssistantBlock`
- `SessionState`
- `AgentLoopResult`
- `run_agent_loop` 的最小文本结束分支

**Step 4: 运行测试确认通过**

Run: `cargo test -p relax-core agent_loop_stops_when_provider_returns_text -- --exact`
Expected: PASS。

**Step 5: 补充文档**

更新 `docs/architecture/agent-loop.md` 中的最小循环说明。

### Task 4: 引入 provider 抽象与 OpenAI-compatible 适配器

**Files:**
- Create: `crates/relax-providers/Cargo.toml`
- Create: `crates/relax-providers/src/lib.rs`
- Create: `crates/relax-providers/src/provider.rs`
- Create: `crates/relax-providers/src/openai_compatible.rs`
- Create: `crates/relax-providers/tests/response_parsing.rs`
- Modify: `docs/plans/2026-03-06-relax-code-agent-design.md`

**Step 1: 写失败测试**

为 OpenAI-compatible 响应解析写测试，覆盖纯文本和工具调用两种响应块。

```rust
#[test]
fn parse_text_response_block() {
    let json = r#"{"choices":[{"message":{"content":"hello"}}]}"#;
    let parsed = parse_chat_response(json).unwrap();
    assert_eq!(parsed.blocks.len(), 1);
}
```

**Step 2: 运行测试确认失败**

Run: `cargo test -p relax-providers parse_text_response_block -- --exact`
Expected: FAIL，因为解析器尚未实现。

**Step 3: 写最小实现**

实现：

- `ChatProvider` trait
- `ProviderRequest` / `ProviderResponse`
- OpenAI-compatible JSON 解析器
- 最小 HTTP 调用封装

**Step 4: 运行测试确认通过**

Run: `cargo test -p relax-providers parse_text_response_block -- --exact`
Expected: PASS。

**Step 5: 记录范围边界**

在设计文档中明确首版仍只支持单一 provider 适配器。

### Task 5: 实现工具 trait、注册表与最小工具集

**Files:**
- Create: `crates/relax-tools/Cargo.toml`
- Create: `crates/relax-tools/src/lib.rs`
- Create: `crates/relax-tools/src/tool.rs`
- Create: `crates/relax-tools/src/registry.rs`
- Create: `crates/relax-tools/src/builtin/shell.rs`
- Create: `crates/relax-tools/src/builtin/read_file.rs`
- Create: `crates/relax-tools/src/builtin/write_file.rs`
- Create: `crates/relax-tools/tests/registry_dispatch.rs`
- Modify: `docs/architecture/tool-system.md`

**Step 1: 写失败测试**

写一个注册表测试，断言按工具名能分发到对应实现。

```rust
#[tokio::test]
async fn registry_dispatches_tool_by_name() {
    let registry = ToolRegistry::with_test_tool("echo_tool");
    let result = registry.invoke("echo_tool", serde_json::json!({"value":"ok"})).await.unwrap();
    assert_eq!(result.output_text(), "ok");
}
```

**Step 2: 运行测试确认失败**

Run: `cargo test -p relax-tools registry_dispatches_tool_by_name -- --exact`
Expected: FAIL，因为工具 trait 与注册表尚未实现。

**Step 3: 写最小实现**

实现：

- `Tool` trait
- `ToolSchema`
- `ToolRegistry`
- `shell`、`read_file`、`write_file` 三个内置工具

**Step 4: 运行测试确认通过**

Run: `cargo test -p relax-tools registry_dispatches_tool_by_name -- --exact`
Expected: PASS。

**Step 5: 补充文档**

将 schema 导出与工具调用约定写入 `docs/architecture/tool-system.md`。

### Task 6: 让 `agent loop` 支持工具调用往返

**Files:**
- Modify: `crates/relax-core/src/agent_loop.rs`
- Create: `crates/relax-core/tests/agent_loop_executes_tool.rs`
- Modify: `crates/relax-core/src/message.rs`
- Modify: `docs/architecture/agent-loop.md`

**Step 1: 写失败测试**

写一个集成级单元测试，断言当 provider 先返回工具调用，再返回文本时，`agent loop` 会执行工具并继续下一轮。

```rust
#[tokio::test]
async fn agent_loop_executes_tool_then_returns_text() {
    let provider = FakeProvider::tool_then_text("read_file", "done");
    let result = run_agent_loop(provider).await.unwrap();
    assert_eq!(result.final_text(), "done");
}
```

**Step 2: 运行测试确认失败**

Run: `cargo test -p relax-core agent_loop_executes_tool_then_returns_text -- --exact`
Expected: FAIL，因为工具回填流程尚未完成。

**Step 3: 写最小实现**

在 `agent_loop` 中加入：

- 工具调用块识别
- 调用 `ToolRegistry`
- 将 `ToolResult` 追加到消息历史
- 再次请求 provider

**Step 4: 运行测试确认通过**

Run: `cargo test -p relax-core agent_loop_executes_tool_then_returns_text -- --exact`
Expected: PASS。

**Step 5: 补充文档**

同步更新 agent loop 数据流图与停止条件。

### Task 7: 实现会话落盘与 `resume` 能力

**Files:**
- Modify: `crates/relax-runtime/src/lib.rs`
- Create: `crates/relax-runtime/src/session_store.rs`
- Create: `crates/relax-runtime/tests/session_roundtrip.rs`
- Modify: `crates/relax-cli/src/main.rs`
- Create: `crates/relax-cli/tests/resume_command.rs`
- Modify: `docs/architecture/session-state.md`

**Step 1: 写失败测试**

先写运行态测试，验证 `SessionState` 可写入磁盘并重新读回。

```rust
#[test]
fn session_state_roundtrip_to_disk() {
    let store = SessionStore::in_temp_dir();
    let session = SessionState::new("demo");
    store.save(&session).unwrap();
    let loaded = store.load("demo").unwrap();
    assert_eq!(loaded.id, "demo");
}
```

**Step 2: 运行测试确认失败**

Run: `cargo test -p relax-runtime session_state_roundtrip_to_disk -- --exact`
Expected: FAIL，因为 `SessionStore` 尚未实现。

**Step 3: 写最小实现**

实现：

- `SessionStore`
- `.relax/sessions/` 目录约定
- `relax resume --session <id>` 最小入口

**Step 4: 运行测试确认通过**

Run: `cargo test -p relax-runtime session_state_roundtrip_to_disk -- --exact`
Expected: PASS。

**Step 5: 增加 CLI 验证**

Run: `cargo test -p relax-cli resume_command_loads_existing_session -- --exact`
Expected: PASS。

### Task 8: 实现 `update_plan` 与本地技能加载

**Files:**
- Create: `crates/relax-tools/src/builtin/update_plan.rs`
- Create: `crates/relax-runtime/src/skill_loader.rs`
- Create: `crates/relax-runtime/tests/skill_loader.rs`
- Modify: `crates/relax-core/src/agent_loop.rs`
- Modify: `crates/relax-cli/src/main.rs`
- Modify: `docs/architecture/prompt-and-skills.md`

**Step 1: 写失败测试**

为本地技能读取写测试，断言能从 `skills/<name>/SKILL.md` 读取内容。

```rust
#[test]
fn skill_loader_reads_skill_markdown() {
    let loader = SkillLoader::from_root("tests/fixtures/skills");
    let skill = loader.load("example").unwrap();
    assert!(skill.contains("# Example Skill"));
}
```

**Step 2: 运行测试确认失败**

Run: `cargo test -p relax-runtime skill_loader_reads_skill_markdown -- --exact`
Expected: FAIL，因为 `SkillLoader` 尚未实现。

**Step 3: 写最小实现**

实现：

- `update_plan` 工具
- `SkillLoader`
- 基础提示组装顺序
- `chat` 命令中的 `--skill` 参数或自动加载入口

**Step 4: 运行测试确认通过**

Run: `cargo test -p relax-runtime skill_loader_reads_skill_markdown -- --exact`
Expected: PASS。

**Step 5: 回归验证**

Run: `cargo test`
Expected: 所有当前已实现测试通过。

### Task 9: 补齐最小用户文档与阶段状态维护

**Files:**
- Modify: `AGENTS.md`
- Modify: `docs/plans/2026-03-06-relax-code-agent-design.md`
- Modify: `docs/architecture/agent-loop.md`
- Modify: `docs/architecture/tool-system.md`
- Modify: `docs/architecture/session-state.md`
- Modify: `docs/architecture/prompt-and-skills.md`

**Step 1: 检查文档与实际实现差异**

逐项对照已完成任务，标记已实现与未实现边界。

**Step 2: 更新阶段说明**

把当前实际进度写回各文档，确保新会话能立即知道做到哪一步。

**Step 3: 执行回归验证**

Run: `cargo test`
Expected: PASS。

**Step 4: 整理交付说明**

记录如何启动 CLI、如何恢复会话、如何继续后续任务。

**Step 5: 记录未完成项**

明确 Stage 5 之后的能力仍为后续范围，不在当前阶段实现。
