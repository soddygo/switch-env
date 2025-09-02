# EnvSwitch Shell 集成说明

## 问题解答

### 为什么需要使用 `eval "$(envswitch use glm)"` 而不能直接用 `envswitch use glm`？

这是一个**技术限制**，不是设计缺陷。原因如下：

1. **子进程限制**：当你运行 `envswitch use glm` 时，它在一个子进程中运行，子进程无法修改父进程（你的shell）的环境变量。

2. **命令生成**：`envswitch use glm` 实际输出的是 shell 命令，比如：
   ```bash
   export ANTHROPIC_BASE_URL='https://api.deepseek.com'
   export ANTHROPIC_MODEL='deepseek-chat'
   export ANTHROPIC_AUTH_TOKEN='sk-your-token'
   ```

3. **eval 执行命令**：`eval "$(envswitch use glm)"` 让你的当前 shell 执行这些 export 命令，从而设置环境变量。

## 解决方案：Shell 集成

我们提供了 shell 集成功能，让你可以直接使用 `envswitch use glm`，无需 `eval`！

### 🚀 快速安装

```bash
# 运行安装脚本
./install-shell-integration.sh

# 重启 shell 或重新加载配置
source ~/.zshrc    # zsh 用户
source ~/.bashrc   # bash 用户
source ~/.config/fish/config.fish  # fish 用户
```

### ✨ 安装后的便捷功能

```bash
# 直接切换配置（无需 eval！）
envswitch use glm

# 使用短别名
esu glm              # envswitch use 的简写
esl                  # envswitch list 的简写
ess                  # envswitch status 的简写

# 交互式选择器
esw                  # 显示配置列表，交互式选择

# 从当前环境快速创建配置
esq myconfig         # 从当前环境变量创建配置
```

### 🔧 工作原理

Shell 集成创建了一个包装函数，它会：
1. 捕获 `envswitch use config` 的输出
2. 自动对输出执行 `eval`
3. 提供无缝的用户体验

这完全安全，只是自动化了你手动要做的事情。

### 📋 支持的 Shell

- **Zsh** (推荐)
- **Bash**
- **Fish**

### 🎯 使用示例

```bash
# 创建配置
envswitch set deepseek \
  -e ANTHROPIC_BASE_URL=https://api.deepseek.com \
  -e ANTHROPIC_MODEL=deepseek-chat \
  -e ANTHROPIC_AUTH_TOKEN=sk-your-token

# 直接切换（安装集成后）
envswitch use deepseek

# 或使用短别名
esu deepseek

# 交互式选择
esw

# 查看状态
ess
```

### 🆘 如果不想安装集成

如果你不想安装 shell 集成，仍然可以使用传统方式：

```bash
# 传统方式（需要 eval）
eval "$(envswitch use deepseek)"

# 或者创建别名
alias use-deepseek='eval "$(envswitch use deepseek)"'
alias use-kimi='eval "$(envswitch use kimi)"'
```

### 🧪 测试安装

运行演示脚本来测试功能：

```bash
./demo.sh
```

这个脚本会：
1. 创建演示配置
2. 安装 shell 集成
3. 演示各种功能
4. 清理演示数据

### 📚 获取帮助

安装集成后，运行以下命令获取帮助：

```bash
envswitch_integration_status
```

这会显示所有可用的功能和使用示例。

## 总结

- **问题**：`envswitch use config` 无法直接设置环境变量（技术限制）
- **传统解决方案**：使用 `eval "$(envswitch use config)"`
- **新解决方案**：安装 shell 集成，直接使用 `envswitch use config`
- **优势**：更简单、更直观、更便捷的用户体验

安装 shell 集成后，你就可以像使用其他命令一样直接使用 envswitch，无需记住复杂的 eval 语法！