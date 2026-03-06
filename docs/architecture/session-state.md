# Session State 架构说明

## 1. 目标

会话状态设计的目标不是单纯保存历史消息，而是让新会话可以真正继续推进工作。

因此，会话状态必须既能恢复“上下文”，也能恢复“当前进度”。

## 2. 目录约定

运行态统一存放在仓库根目录下的 `.relax/`：

```text
.relax/
├─ sessions/
├─ tasks/
├─ cache/
└─ skills-index.json
```

说明：

- `sessions/`：保存会话 JSON 文件。
- `tasks/`：保存任务状态与计划状态。
- `cache/`：保存 provider 或技能缓存。
- `skills-index.json`：保存技能索引缓存。

当前实现通过 `RuntimePaths::from_workspace(...)` 统一计算以下路径：

- `root`
- `sessions`
- `tasks`
- `cache`
- `skills_index`

`SessionStore` 当前会对 `session_id` 做最小安全校验，只允许 ASCII 字母数字、`-` 和 `_`，避免路径逃逸出 `.relax/sessions/`。

## 3. SessionState 建议字段

- `id`
- `created_at`
- `updated_at`
- `workspace_root`
- `provider_name`
- `model_name`
- `messages`
- `active_skills`
- `plan_state`
- `summary`

## 4. 首版恢复行为

`resume` 时至少恢复：

- 历史消息
- 当前 provider / model
- 当前工作目录
- 当前计划
- 当前摘要

恢复后可继续进入 `chat` 循环。

首版先提供最小命令入口：

- `relax resume --session <id> --workspace <path>`

该入口当前只验证会话可以被读取并恢复到内存，不要求立即接回完整 chat loop。

当前 CLI 恢复行为会输出已加载的会话 ID 与消息数量，用于验证最小恢复链路已经成立。

## 5. 首版序列化与配置约定

- 存储格式：JSON
- 序列化库：`serde` + `serde_json`
- 文件命名：`<session-id>.json`
- 工作区配置文件：`relax.toml`
- 默认 provider：`openai-compatible`
- 默认 model：`gpt-4.1-mini`
- 环境变量覆盖：`RELAX_PROVIDER`、`RELAX_MODEL`

文件默认落在：

- `.relax/sessions/<session-id>.json`

首版优先保证可读、可调试、可手工检查，不急于引入数据库。

## 6. 决策状态的补充

仅靠 `SessionState` 不足以支撑长期续做，因此还必须依赖：

- `AGENTS.md`
- `docs/plans/*.md`
- `docs/architecture/*.md`

这三类文档共同构成项目级长期记忆。

当前仍待补齐的部分：

- 更完整的 `SessionState` 字段持久化仍是后续工作。
- “恢复后继续执行完整 chat loop” 仍未落地。
- 会话损坏、会话不存在等错误路径测试还可以继续补强。
