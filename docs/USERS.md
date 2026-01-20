# SeleneJS（用户文档）

SeleneJS 是一个轻量的前端 UI 框架实验项目，提供：
- **响应式状态**：`signal / effect / computed`
- **渲染**：`h / render / reactiveRender`
- **可选模板编译**：`compileTemplate`（Rust/WASM）把模板字符串编译成渲染函数

> 适用场景：学习/实验、小型 demo、探索 “Rust/WASM 编译器 + JS runtime” 的组合。

## 用法模式总览

SeleneJS 提供两种常见“模式”：

- **纯 JS 模式**：只用 `signal / effect / h / createApp`，不关心 Rust/WASM，跟普通 JS 框架一样简单。
- **Rust 增强模式**：在 JS 模式基础上，选择性使用 Rust 编译器和 Rust signal，获得更高性能或更多能力。

下面先看 JS 模式，再看如何在实际项目中“打开 Rust 按钮”。

## 1. JS 模式（最简单的用法）

### 1.1 创建状态

- `signal(initial)`：创建可变状态
- `effect(fn)`：自动追踪依赖并在变化时重新执行
- `computed(fn)`：派生值

### 1.2 渲染（两种方式）

#### 方式 A：直接用 `h` 写 UI（推荐初学）

- `h(tagOrComponent, props, ...children)` 创建 VNode
- `reactiveRender(() => VNode, container)` 自动追踪依赖并更新 UI

#### 方式 B：用模板编译（进阶）

- `compileTemplate(templateString)` 返回一个 JS 函数源码字符串（例如 `() => h(...)`）
- 你可以用 `new Function(...)` 将其转成真正的渲染函数（示例里演示了如何把 `h/Fragment` 注入进去）

> 注意：模板里包含 `${...}` 时，需要写成 `\${...}` 才会在编译后在运行时读取信号值。

### 1.3 `createApp`：更贴合 JS 思维的挂载方式

```js
import { signal, h, createApp } from '@selene/core'

const count = signal(0)

const App = () =>
  h('div', { class: 'counter' }, [
    h('p', {}, `Count: ${count.value}`),
    h('button', { onClick: () => count.value++ }, 'Increment'),
  ])

createApp(App).mount('#app')
```

## 2. Rust 增强模式：在 JS 代码里“开 Rust”

可以在“正常 JS 写法”的基础上，按需引入 Rust 能力：

### 2.1 Rust 信号（createRustSignal）

```js
import { signal, createRustSignal, effect } from '@selene/core'

const jsCount = signal(0)

async function main() {
  // Rust 实现的 Signal（内部在 WASM 里维护值）
  const rustCount = await createRustSignal(0)

  effect(() => {
    console.log('jsCount =', jsCount.value)
  })

  // Rust signal 直接用 .value 读写
  rustCount.value = 1
  console.log('rustCount =', rustCount.value)
}

main()
```

特点：
- API 与 JS `signal` 完全一致（`.value` 读写），只是实现搬到 Rust/WASM。
- 你可以把热点状态迁移到 Rust，用 JS 做壳和 UI。

### 2.2 Rust 模板编译（compileTemplate）

`examples/basic/src/main.js` 展示了：
- 使用 `compileTemplate`（Rust/WASM 优先，JS 为后备）把模板编成 `() => h(...)` 的函数源码
- 用 `new Function('h','Fragment','count','name', ...)` 把源码变成真正的渲染函数
- 用 `createApp(() => Counter())` 做挂载和自动更新

---

## 安装与运行（Installation & Run）

1) 开发环境（推荐）

```bash
# 安装依赖
pnpm install

# 在需要时构建 packages（或使用 monorepo 的脚本）
pnpm -w build
```

2) 运行示例（本地查看）：

```bash
cd examples/basic
python3 -m http.server 8000
# 或
npx http-server -c-1 -p 8000
```

3) 打包/发布（示意）

```bash
# 各 package 的打包命令参考 packages/*/package.json
pnpm -w build
# 然后按需发布到 npm
pnpm -w publish --access public
```

---

## English Quick Start

1) Install deps:

```bash
pnpm install
pnpm -w build
```

2) Serve the basic example:

```bash
cd examples/basic
python3 -m http.server 8000
# or
npx http-server -c-1 -p 8000
```

Open `http://localhost:8000` and edit `examples/basic/src/main.js` to iterate.

---

## FAQ（补充）

Q: 我应该什么时候构建 WASM？
A: 仅在你修改或依赖 Rust 端的逻辑时需要构建；普通前端开发只需运行 JS 的构建流程并打开 `examples/` 即可。

Q: 如何把示例改为加载 `dist`？
A: 把 `examples/basic/index.html` 中的 `<script src="/src/main.js">` 替换为指向打包好的 `dist` 文件（例如 `/dist/index.js`）并确保静态服务器能提供 `dist` 目录。

## 与其他框架对比：这个项目的优势

- **相对 React**
  - **优势**：概念更少（signal/effect/h）；更容易把“模板 → 渲染函数”放到编译期；可以探索 Rust/WASM 作为编译基础设施
  - **现状差距**：缺少成熟的组件生态、DevTools、性能优化与边界处理

- **相对 Vue**
  - **优势**：内核更小；编译器可用 Rust/WASM 做跨平台复用
  - **现状差距**：Vue 的模板语法、编译优化、运行时能力更完整

- **相对 Solid**
  - **优势**：同样可以走 “fine-grained + compiler 优化” 的路线；signal 模型更接近
  - **现状差距**：Solid 的更新粒度、编译期优化更成熟

- **相对 Svelte**
  - **优势**：同样是 compiler-first 思想；Rust/WASM 可能更适合做“编译服务/跨端编译”
  - **现状差距**：Svelte 的编译产物质量与工具链成熟度更高

## 常见问题

### Q: 为什么示例里要用 `new Function`？
因为目前 `compileTemplate` 的输出是“函数源码字符串”。后续更推荐把编译产物做成 **可直接 import 的 ESM 模块**，避免运行时 eval。

### Q: 浏览器/Node 都能用吗？
可以。`@selene/core` 构建时会把 wasm 资源复制到 `dist/rust`，让打包后运行时能正确找到 wasm。

---

## English translations & Quick Start

### What is SeleneJS?
SeleneJS is a minimal experimental UI framework combining a JS runtime (signals, effects, rendering) with an optional Rust/WASM template compiler.

### Quick Start (Run the basic example)
1. Open a terminal and serve `examples/basic` as static files:

```bash
cd examples/basic
python3 -m http.server 8000
# or
npx http-server -c-1 -p 8000
```

2. Open `http://localhost:8000` in your browser and inspect the example. `examples/basic/index.html` loads the unbundled source via `<script type="module" src="/src/main.js">` so you can iterate on `examples/basic/src/main.js` directly.

### Notes
- `compileTemplate` currently returns function source code (string). Example demonstrates turning that string into a callable function via `new Function(...)`. Future improvements aim to produce ESM artifacts to avoid runtime `eval`.
- For packaging and production builds, follow the build scripts in `packages/*/package.json` and ensure any wasm artifacts are copied into `dist/rust`.


