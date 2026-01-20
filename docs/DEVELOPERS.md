# SeleneJS（开发者文档）

本仓库是一个**以“轻量 runtime + 可选 Rust/WASM 加速编译器”**为核心思路的前端 UI 框架实验项目。

## 项目结构

- `packages/core/`: `@selene/core`，运行时核心（响应式、渲染、JSX 支持、WASM bindings）
- `crates/core/`: Rust crate（导出到 WASM 的 signal / render / compiler 相关能力）
- `examples/basic/`: 示例应用（Vite + `@selene/core`）
- `docs/`: 文档（本目录）

## 当前能力（截至目前）

### Runtime（TypeScript）
- **响应式**：`signal / effect / computed`
- **渲染**：`h`（VNode 工厂）+ `render`（直接 DOM 渲染）+ `reactiveRender`（依赖追踪后自动重渲染）
- **JSX**：提供 `jsx/jsxs` 入口以及 `Fragment`

### 编译器（Rust/WASM）
- `crates/core/src/compiler.rs` 提供 `compile_template`，输出一个 JS 函数源码字符串：
  - 形如：`"() => h(\"div\", {...}, [...])"`
  - 支持：嵌套元素、自闭合/void 标签、属性（boolean/string/`{expr}`）、文本与模板字符串（`${}`）
  - 多根节点会输出 `h(Fragment, {}, [...])`（要求运行时 eval 环境能访问 `Fragment`）

### 构建/发布
- `packages/core/scripts/copy-rust.mjs`：构建后把 `src/rust` 复制到 `dist/rust`，保证 Node/打包后运行时能找到 wasm 文件。

## 设计取舍与“删掉不必要部分”的原则

### 原则
- **只保留一条可信路径**：模板编译走 Rust/WASM；JS fallback 仅作为“环境不支持 WASM”时的 best-effort。
- **职责分层**：runtime 负责“渲染 + 响应式”；compiler 负责“模板 → 渲染函数源码”。
- **示例不承载框架逻辑**：示例里避免写“编译器替身/构建 hack/复杂 fallback”，这些应该在 compiler/runtime 内部完成。

### 已清理
- 移除 `tools/swc-build.js`（build-time 的模板预编译 hack），避免重复实现与维护。

## Roadmap（建议的下一步）

### 1) 拆分包：`@selene/compiler`
当前 `packages/compiler/` 为空，建议落地成一个真正的包：
- Node 侧：直接调用 Rust（CLI 或 native）或使用 wasm 的 node target
- Browser 侧：使用 wasm 的 web target

目标：`@selene/core` 只保留 runtime；`compileTemplate` 迁移/重导出到 `@selene/compiler`（或在 core 里做薄封装）。

### 2) 模板语法升级
当前编译器是 HTML-ish：
- 增强点：事件语法（如 `on:click={...}` 或 `@click={...}`）、绑定/插槽、组件标签识别、静态提升
- 安全点：减少/去除 `eval/new Function` 依赖（编译期产物直接输出 JS module，而不是函数源码字符串）

### 3) 渲染与 diff
目前是“每次 render 清空 container”，适合 demo：
- 引入 diff/patch（最小化 DOM 变更）
- Fragment/数组 children 的更一致行为（包含 `null/false` 等）

## 与其他框架的对比（优势与定位）

> 这个项目当前更像“学习/实验型内核”，优势主要来自“极简 API + Rust/WASM 编译器探索”。

- **对比 React**
  - **优势**：API 面更小；无需 hooks 规则；可走编译器路线把模板变成高效的渲染函数
  - **劣势/待补**：生态、开发工具、diff 性能与最佳实践还远不如 React

- **对比 Vue**
  - **优势**：可以更激进地把 compiler 放到 Rust/WASM；核心实现更小更易读
  - **劣势/待补**：Vue 的模板语法/编译优化/运行时边界处理非常成熟，本项目还处于早期

- **对比 Solid**
  - **优势**：可以向 Solid 靠拢（“编译期确定依赖 + 最小更新”），并通过 Rust 编译器探索更多优化空间
  - **劣势/待补**：Solid 的 fine-grained 更新与成熟编译 pipeline 目前还没实现

- **对比 Svelte**
  - **优势**：同样是 compiler-first 的方向；Rust/WASM 可能在跨平台编译上更有想象空间
  - **劣势/待补**：Svelte 的“无 runtime/极小 runtime”与编译产物质量，目前仍是目标而非现状

---

## 本文件包含（快速导航）
- 项目简介与目标（中/EN）
- 本地开发与构建流程（含 WASM 构建）
- 发布/打包注意事项
- 代码结构与包说明

（若需更详细 API，请参见 `docs/API.md`；示例请见 `docs/EXAMPLES.md`）

---

## 本地开发（详细步骤 / Local dev steps）

1. 克隆仓库并安装依赖：

```bash
git clone <repo-url>
cd SeleneJS
pnpm install
```

2. 运行 TypeScript/JS 构建（若要 watch）：

```bash
pnpm --filter packages/core dev
# 或者参考各 package 的 package.json scripts
```

3. 构建/调试 Rust -> WASM（可选，仅在需要编译器/wasm 功能时）

```bash
# 以 crates/core 为例，具体 build 流程视项目设置而定
cd crates/core
cargo build --release --target wasm32-unknown-unknown
# 若需要 wasm-bindgen 或 wasm-pack，请在此基础上运行相应命令
```

4. 将 wasm 复制到 runtime 可访问位置（示例项目使用 packages/core/scripts/copy-rust.mjs）：

```bash
node packages/core/scripts/copy-rust.mjs
```

5. 本地查看示例（在 `examples/basic` 下启动静态服务器）：

```bash
cd examples/basic
python3 -m http.server 8000
# 或者
npx http-server -c-1 -p 8000
```

---

## 发布/打包注意（Release notes / Packaging hints）

- 确保 wasm artifacts 被复制到 `dist/rust`，并在构建后校验静态文件是否随包一起发布。
- 若希望发布单独的 `@selene/compiler` 包，请在发布前把 compileTemplate 的 Node/Browser 入口与 wasm binding 明确分离。

---

更多细节与贡献指南见 `docs/CONTRIBUTING.md`。
---

## English translations & quick reference

### English summary
This repository implements a small JS runtime plus an optional Rust/WASM compiler. The goal is to explore compiler-first approaches and Rust/WASM integration for template compilation.

### Project layout (quick)
- `packages/core/`: runtime (`@selene/core`) — reactivity, rendering, JSX glue, wasm helpers
- `crates/core/`: Rust sources that can compile to WASM (compiler, low-level primitives)
- `examples/`: runnable demos (see `examples/basic`)

### Design trade-offs
- Keep runtime minimal; push template-to-renderer work into compiler.
- Prefer single clear workflow: Rust/WASM compiler + minimal JS runtime; JS fallback only when necessary.

### Quick start for developers
Prerequisites: `node`, a package manager (pnpm/yarn/npm), and Rust toolchain if you plan to build the WASM artifacts.

Run an example locally (serve static files from `examples/basic`):

```bash
cd examples/basic
# using Python http server
python3 -m http.server 8000

# or using npx http-server
npx http-server -c-1 -p 8000
```

Build Rust (optional, for WASM/advanced builds):

```bash
cd crates/core
cargo build --release --target wasm32-unknown-unknown
# Note: additional wasm-bindgen/wasm-pack steps may be required depending on your packaging
```

Refer to `packages/*/package.json` scripts for JS build steps and `packages/core/scripts/copy-rust.mjs` for wasm copy logic.


