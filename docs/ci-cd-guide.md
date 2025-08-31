# CI/CD 指南

本项目使用 GitHub Actions 实现自动化的持续集成和持续部署。

## 工作流概述

### 1. CI 工作流 (`.github/workflows/ci.yml`)

**触发条件：**
- 推送到 `main` 或 `develop` 分支
- 创建针对 `main` 或 `develop` 分支的 Pull Request

**功能：**
- 在多个操作系统（Ubuntu、macOS、Windows）上运行测试
- 使用 stable 和 beta 版本的 Rust 进行测试
- 代码格式检查 (`cargo fmt`)
- 代码质量检查 (`cargo clippy`)
- 安全审计 (`cargo audit`)
- 代码覆盖率报告（推送到 Codecov）

### 2. Release 工作流 (`.github/workflows/release.yml`)

**触发条件：**
- 推送以 `v` 开头的标签（如 `v1.0.0`）
- 手动触发（workflow_dispatch）

**功能：**
- 为多个平台编译二进制文件：
  - Linux x86_64 (glibc)
  - Linux x86_64 (musl, 静态链接)
  - Linux ARM64
  - macOS x86_64
  - macOS ARM64 (Apple Silicon)
  - Windows x86_64
  - Windows ARM64
- 生成 SHA256 校验和
- 自动创建 GitHub Release
- 上传所有二进制文件和校验和文件

## 发布流程

### 自动发布

1. **准备发布：**
   ```bash
   # 确保在 main 分支且工作目录干净
   git checkout main
   git pull origin main
   
   # 使用发布脚本
   ./scripts/release.sh v1.0.0
   ```

2. **发布脚本会自动：**
   - 更新 `Cargo.toml` 中的版本号
   - 更新 `Cargo.lock`
   - 提交版本变更
   - 创建并推送标签
   - 触发 GitHub Actions 发布流程

### 手动发布

1. **通过 GitHub 界面：**
   - 访问 Actions 页面
   - 选择 "Release" 工作流
   - 点击 "Run workflow"
   - 输入版本号（如 `v1.0.0`）

2. **通过命令行：**
   ```bash
   # 创建标签
   git tag v1.0.0
   git push origin v1.0.0
   ```

## 支持的平台

| 平台 | 架构 | 文件名 | 说明 |
|------|------|--------|------|
| Linux | x86_64 | `envswitch-linux-x86_64` | 动态链接，需要 glibc |
| Linux | x86_64 | `envswitch-linux-x86_64-musl` | 静态链接，无依赖 |
| Linux | ARM64 | `envswitch-linux-aarch64` | 适用于 ARM64 Linux |
| macOS | x86_64 | `envswitch-macos-x86_64` | Intel Mac |
| macOS | ARM64 | `envswitch-macos-aarch64` | Apple Silicon Mac |
| Windows | x86_64 | `envswitch-windows-x86_64.exe` | 64位 Windows |
| Windows | ARM64 | `envswitch-windows-aarch64.exe` | ARM64 Windows |

## 配置要求

### GitHub Secrets

确保仓库配置了以下 secrets：

- `GITHUB_TOKEN`: GitHub 自动提供，用于创建 release
- `CODECOV_TOKEN`: （可选）用于上传代码覆盖率报告

### 权限设置

确保 GitHub Actions 有以下权限：
- Contents: Write（用于创建 release）
- Actions: Read（用于运行工作流）

## 本地开发

### 运行测试
```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test --test integration_tests

# 运行带覆盖率的测试
cargo tarpaulin --all-features
```

### 代码质量检查
```bash
# 格式化代码
cargo fmt

# 运行 clippy
cargo clippy

# 安全审计
cargo audit
```

### 本地构建
```bash
# Debug 构建
cargo build

# Release 构建
cargo build --release

# 交叉编译（需要安装对应工具链）
cargo build --target x86_64-unknown-linux-musl
```

## 故障排除

### 常见问题

1. **编译失败：**
   - 检查 Rust 版本兼容性
   - 确保所有依赖都能正确解析
   - 查看具体的编译错误信息

2. **测试失败：**
   - 检查测试环境是否正确设置
   - 确保测试数据文件存在
   - 查看测试日志获取详细信息

3. **发布失败：**
   - 检查标签格式是否正确（必须以 `v` 开头）
   - 确保有足够的权限创建 release
   - 检查 GitHub Actions 日志

### 调试技巧

1. **查看 Actions 日志：**
   - 访问 GitHub 仓库的 Actions 页面
   - 点击失败的工作流查看详细日志

2. **本地复现问题：**
   ```bash
   # 使用相同的 Rust 版本
   rustup install stable
   rustup default stable
   
   # 清理并重新构建
   cargo clean
   cargo build
   ```

3. **检查依赖：**
   ```bash
   # 更新依赖
   cargo update
   
   # 检查过时的依赖
   cargo outdated
   ```

## 最佳实践

1. **版本管理：**
   - 使用语义化版本（Semantic Versioning）
   - 在 CHANGELOG.md 中记录变更
   - 为重要版本创建 release notes

2. **测试策略：**
   - 保持高测试覆盖率
   - 包含集成测试和单元测试
   - 在多个平台上测试

3. **安全考虑：**
   - 定期运行安全审计
   - 及时更新依赖
   - 使用最小权限原则

4. **性能优化：**
   - 使用缓存加速构建
   - 并行运行独立的任务
   - 优化 Docker 镜像大小（如果使用）