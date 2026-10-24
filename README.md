# GPUI Todo

基于 [GPUI](https://github.com/zed-industries/zed/tree/main/crates/gpui)（Zed 编辑器的 Rust UI 框架）编写的 Todo 演示应用。

## 功能

- 点击「+ 添加待办」新增待办事项
- 点击待办标题切换完成状态
- 点击「删除」移除待办
- 底部实时显示未完成数量

## 环境要求

- Rust 1.85+（项目使用 2024 edition）
- macOS（GPUI 目前对 macOS 支持最完善）

安装 Rust：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## 启动

```bash
cargo run
```

首次运行会下载并编译依赖，之后再次启动会很快。

## 打包 macOS 应用

执行打包脚本，会在 `dist/` 目录下生成 `GPUI Todo.app`：

```bash
./scripts/package-macos.sh
```

生成后可直接双击运行，或用命令打开：

```bash
open "dist/GPUI Todo.app"
```

## 项目结构

```
src/main.rs               # 应用入口与全部 UI 逻辑
scripts/package-macos.sh  # macOS .app 打包脚本
dist/                     # 打包产物输出目录
```
