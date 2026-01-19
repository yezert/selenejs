# SeleneJS（用户文档）

SeleneJS 是一个轻量的前端 UI 框架实验项目，提供：
- **响应式状态**：`signal / effect / computed`
- **渲染**：`h / render / reactiveRender`
- **可选模板编译**：`compileTemplate`（Rust/WASM）把模板字符串编译成渲染函数

> 适用场景：学习/实验、小型 demo、探索 “Rust/WASM 编译器 + JS runtime” 的组合。

## 快速上手

### 1) 创建状态

- `signal(initial)`：创建可变状态
- `effect(fn)`：自动追踪依赖并在变化时重新执行
- `computed(fn)`：派生值

### 2) 渲染（两种方式）

#### 方式 A：直接用 `h` 写 UI（推荐初学）

- `h(tagOrComponent, props, ...children)` 创建 VNode
- `reactiveRender(() => VNode, container)` 自动追踪依赖并更新 UI

#### 方式 B：用模板编译（进阶）

- `compileTemplate(templateString)` 返回一个 JS 函数源码字符串（例如 `() => h(...)`）
- 你可以用 `new Function(...)` 将其转成真正的渲染函数（示例里演示了如何把 `h/Fragment` 注入进去）

> 注意：模板里包含 `${...}` 时，会以 JS 模板字符串形式保留；要实现真正的“响应式模板”，需要确保在渲染函数执行时读取信号值。

## 示例：计数器

`examples/basic/src/main.js` 展示了：
- 使用 `compileTemplate` 编译模板
- 使用 `reactiveRender` 自动更新
- 使用事件委托（`data-action`）绑定点击逻辑

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

