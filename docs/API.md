# API Reference / 接口参考

本页为 `@selene/core` 的主要接口速查（概要）。详细实现请查看 `packages/core/src`。

## 核心概念 / Core concepts

- `signal(initial)` — 创建一个响应式信号；访问 `.value` 读取或写入。
- `effect(fn)` — 追踪依赖信号并在变化时重新执行 `fn`。
- `computed(fn)` — 基于 signal 的派生值，只有在依赖变更时重新计算。
- `h(tagOrComponent, props, ...children)` — 创建 VNode。用于手写渲染函数或编译产物。
- `reactiveRender(fn, container)` — 自动追踪 `fn` 中访问的信号，渲染返回的 VNode 到 `container`。
- `createApp(view)` — 类 Vue 的 API，封装了 `reactiveRender`，通过 `.mount('#app')` 挂载。

## 模板编译器（compileTemplate）

- `compileTemplate(templateString) -> Promise<string>`：将模板字符串编译为渲染函数源码（string）。目前输出需要通过 `new Function(...)` 或打包后作为 ESM 使用。

示例：

```js
const src = await compileTemplate('<div>${count.value}</div>')
const render = new Function('h','Fragment','count', `return (${src});`)(h, Fragment, count)
```

## 其它工具 / utilities

- `Fragment` — 多根节点占位符（用于编译输出中的多根）
- `createRustSignal(initialValue)` — 使用 Rust/WASM 实现的 Signal（异步，返回 Promise）。

## 注意 / Notes

- `compileTemplate` 当前输出的是函数源码字符串；未来目标是输出 ESM 模块或更安全的产物，减少运行时 `eval`。

更多细节请阅读 `packages/core/src` 源码与对应导出的类型声明 `packages/core/src/rust.d.ts`。
