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

## 5. 首版序列化策略

- 存储格式：JSON
- 序列化库：`serde` + `serde_json`
- 文件命名：`<session-id>.json`

首版优先保证可读、可调试、可手工检查，不急于引入数据库。

## 6. 决策状态的补充

仅靠 `SessionState` 不足以支撑长期续做，因此还必须依赖：

- `AGENTS.md`
- `docs/plans/*.md`
- `docs/architecture/*.md`

这三类文档共同构成项目级长期记忆。
