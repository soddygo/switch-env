# Requirements Document

## Introduction

这是一个用Rust开发的命令行工具，用于管理和快速切换不同的环境变量配置。主要用于在使用Claude Code工具时快速切换不同的AI模型配置（如DeepSeek v3.1和Kimi k2），但设计为通用工具，支持任意环境变量的配置和切换。工具需要支持zsh、fish、bash等主流shell。

## Requirements

### Requirement 1

**User Story:** 作为开发者，我想要能够保存不同的环境变量配置集合并给它们起别名，这样我就可以轻松管理多套配置。

#### Acceptance Criteria

1. WHEN 用户运行配置命令 THEN 系统 SHALL 允许用户创建新的配置别名
2. WHEN 用户创建配置 THEN 系统 SHALL 允许用户设置多个环境变量键值对
3. WHEN 用户保存配置 THEN 系统 SHALL 将配置持久化存储到本地文件
4. WHEN 用户查看配置列表 THEN 系统 SHALL 显示所有已保存的配置别名

### Requirement 2

**User Story:** 作为开发者，我想要能够快速切换到指定的环境变量配置，这样我就可以在不同的AI模型之间无缝切换。

#### Acceptance Criteria

1. WHEN 用户运行切换命令并指定别名 THEN 系统 SHALL 设置对应的环境变量
2. WHEN 切换配置 THEN 系统 SHALL 在当前shell会话中生效
3. WHEN 切换成功 THEN 系统 SHALL 显示确认信息和当前激活的配置
4. IF 指定的别名不存在 THEN 系统 SHALL 显示错误信息并列出可用别名

### Requirement 3

**User Story:** 作为开发者，我想要工具支持多种shell环境，这样我就可以在我偏好的shell中使用这个工具。

#### Acceptance Criteria

1. WHEN 工具在zsh中运行 THEN 系统 SHALL 正确设置环境变量
2. WHEN 工具在fish中运行 THEN 系统 SHALL 正确设置环境变量
3. WHEN 工具在bash中运行 THEN 系统 SHALL 正确设置环境变量
4. WHEN 检测到不支持的shell THEN 系统 SHALL 显示警告但尝试使用通用方法

### Requirement 4

**User Story:** 作为开发者，我想要能够编辑和删除已有的配置，这样我就可以维护我的配置集合。

#### Acceptance Criteria

1. WHEN 用户运行编辑命令 THEN 系统 SHALL 允许修改指定别名的配置
2. WHEN 用户运行删除命令 THEN 系统 SHALL 移除指定的配置别名
3. WHEN 删除配置前 THEN 系统 SHALL 要求用户确认操作
4. WHEN 编辑不存在的配置 THEN 系统 SHALL 提示是否创建新配置

### Requirement 5

**User Story:** 作为开发者，我想要能够查看当前激活的配置和环境变量状态，这样我就可以确认当前的设置。

#### Acceptance Criteria

1. WHEN 用户运行状态查看命令 THEN 系统 SHALL 显示当前激活的配置别名
2. WHEN 显示状态 THEN 系统 SHALL 列出当前设置的相关环境变量值
3. WHEN 没有激活配置 THEN 系统 SHALL 显示"无激活配置"状态
4. WHEN 查看状态 THEN 系统 SHALL 显示配置文件位置信息

### Requirement 6

**User Story:** 作为开发者，我想要工具提供清晰的帮助信息和错误提示，这样我就可以快速学会使用这个工具。

#### Acceptance Criteria

1. WHEN 用户运行help命令 THEN 系统 SHALL 显示所有可用命令和用法示例
2. WHEN 用户输入错误命令 THEN 系统 SHALL 显示有用的错误信息和建议
3. WHEN 发生配置文件错误 THEN 系统 SHALL 提供清晰的错误描述和解决建议
4. WHEN 首次使用 THEN 系统 SHALL 提供快速入门指导

### Requirement 7

**User Story:** 作为开发者，我想要能够导入和导出配置，这样我就可以在不同机器间同步配置或备份配置。

#### Acceptance Criteria

1. WHEN 用户运行导出命令 THEN 系统 SHALL 将配置导出为JSON或YAML格式
2. WHEN 用户运行导入命令 THEN 系统 SHALL 从文件导入配置
3. WHEN 导入配置与现有配置冲突 THEN 系统 SHALL 询问用户如何处理
4. WHEN 导入文件格式错误 THEN 系统 SHALL 显示详细的格式错误信息