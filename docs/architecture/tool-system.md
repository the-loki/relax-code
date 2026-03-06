# Tool System 架构说明

## 1. 目标

工具系统负责把模型的“想做什么”转成宿主环境中的“可执行动作”。它必须具备统一 schema、统一调用入口和统一结果格式。

## 2. 设计原则

- 工具以 trait 形式注册，不把工具逻辑写死在 `agent loop` 中。
- 工具必须能导出 schema，供 provider 一起提交给模型。
- 工具返回结构必须统一，方便消息回填与日志记录。
- 首版先追求稳定和可测试，不追求插件市场式动态加载。

## 3. 推荐抽象

### 3.1 Tool trait

建议最少包含：

- `name()`：工具名
- `description()`：工具说明
- `input_schema()`：输入 schema
- `invoke()`：执行工具

### 3.2 ToolRegistry

负责：

- 注册工具
- 导出全部工具 schema
- 根据工具名分发调用
- 处理“工具不存在”错误

当前最小实现约定：

- 注册表按工具名维护工具实例。
- 注册表可导出全部 `ToolSchema`，供后续 provider 层组装工具声明。
- 注册表通过统一 `invoke(name, input)` 入口分发工具调用。

### 3.3 ToolResult

建议至少包含：

- `tool_name`
- `success`
- `content`
- `metadata`

首版最小实现中，`ToolResult` 先只稳定输出文本内容，后续再扩展结构化 metadata。

## 4. 首版工具清单

- `shell`
  - 用于执行本地命令
- `read_file`
  - 读取文件内容
- `write_file`
  - 写入文件内容
- `update_plan`
  - 写入或更新当前计划状态

当前已落地的最小工具调用约定：

- `shell`
  - 输入：`{"command": "..."}`
- `read_file`
  - 输入：`{"path": "..."}`
- `write_file`
  - 输入：`{"path": "...", "content": "..."}`
- `update_plan`
  - 输入：最小 JSON 计划结构
  - 当前行为：格式化并返回计划文本，不直接持久化状态

## 5. 首版边界

首版不做：

- 浏览器自动化工具
- 网络搜索工具
- 多命名空间工具协议
- 复杂审批机制

需要先把本地 coding agent 的基本闭环跑通。

当前还未闭环的部分：

- `update_plan` 仍是最小占位实现，还没有接到真实计划持久化层。
- 注册表导出的 schema 顺序目前不保证稳定；后续若 provider 侧依赖稳定顺序，需要再收敛实现。

## 6. 安全边界

后续需要为工具系统补充：

- 工作目录限制
- 路径安全校验
- shell 命令策略
- 危险操作确认

但这些不应阻塞首版骨架搭建。
