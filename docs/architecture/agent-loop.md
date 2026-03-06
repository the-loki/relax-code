# Agent Loop 架构说明

## 1. 目标

`agent loop` 是整个系统中最稳定的核心。后续不论增加工具、技能、任务、subagent 还是上下文压缩，都应尽量作为外部能力接入，而不是反复重写循环本身。

## 2. 最小循环

```text
用户输入
  -> 组装运行上下文
  -> 调用 provider
  -> 若返回普通文本，则结束
  -> 若返回工具调用，则执行工具
  -> 将工具结果回填到消息历史
  -> 再次调用 provider
  -> 直到没有新的工具调用
```

## 3. 核心对象

- `Message`
  - 表示系统、用户、助手、工具结果等消息。
- `AssistantBlock`
  - 表示助手返回的文本块或工具调用块。
- `ToolCall`
  - 表示模型发起的工具调用请求。
- `ToolResult`
  - 表示工具返回结果。
- `SessionState`
  - 表示一个可恢复会话的完整状态。
- `TurnContext`
  - 表示当前回合的运行上下文，如工作目录、技能、计划、配置。

## 4. 依赖关系

`agent loop` 只依赖抽象，不直接依赖具体实现：

- 依赖 `ChatProvider` trait，而不是具体 OpenAI 客户端。
- 依赖 `ToolRegistry` 抽象，而不是具体 `shell` 或 `read_file` 实现。
- 依赖 `SessionStore` 或 runtime 接口，而不是具体磁盘格式。

## 5. 停止条件

首版建议支持以下停止条件：

- provider 返回纯文本且不再要求工具调用。
- 超过最大工具调用轮次。
- 工具执行出错且策略要求终止。
- 用户主动中断。

## 6. 首版不做的复杂逻辑

- 并行工具调用
- 多 provider 回退
- 复杂重试策略
- context compaction
- subagent 调度

这些能力后续都通过 loop 周边模块扩展，而不是推翻 loop 本身。
