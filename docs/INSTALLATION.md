# Installation / 安装

本页面介绍如何在本地设置开发环境并构建项目（中/EN）。

## 前置条件 / Prerequisites
- Node.js (建议 v16+)
- pnpm / npm / yarn（文档示例使用 `pnpm`）
- Rust toolchain（可选，仅当你需要构建 WASM/修改 Rust 代码时）

## 本地安装（中文）

1. 克隆仓库：

```bash
git clone <repo-url>
cd SeleneJS
```

2. 安装 JS 依赖：

```bash
pnpm install
```

3. 构建所有包（如需）：

```bash
pnpm -w build
```

4. 可选：构建 Rust -> wasm：

```bash
cd crates/core
cargo build --release --target wasm32-unknown-unknown
# 如需 wasm-bindgen:
# wasm-bindgen target/wasm32-unknown-unknown/release/<name>.wasm --out-dir pkg --target web
```

## Local dev (English)

1. Clone and install:

```bash
git clone <repo-url>
cd SeleneJS
pnpm install
```

2. Build monorepo (optional):

```bash
pnpm -w build
```

3. Serve examples:

```bash
cd examples/basic
python3 -m http.server 8000
```

Notes: wasm build is optional for most frontend iteration cycles; only required when changing Rust code.
