# Guides / 教程

本节提供快速教程与实践指南（中/EN）。

## 1) 从 signal 到 UI（示例流程）

1. 在 `examples/basic/src/main.js` 中创建 signal：

```js
import { signal, reactiveRender, h } from '@selene/core'
const count = signal(0)
reactiveRender(() => h('div', null, `Count: ${count.value}`), document.getElementById('app'))
```

2. 运行示例并在浏览器观察自动更新。

## 2) 使用模板编译器（实验性）

1. 用 `compileTemplate` 编译字符串模板，得到函数源码字符串。
2. 通过 `new Function(...)` 将源码转成可执行渲染函数（示例已在 `examples/basic/src/main.js`）。

## 3) 打包与发布（建议流程）

1. 在 monorepo 根运行 `pnpm -w build`。
2. 检查 `dist/`，确保 `dist/rust` 包含 wasm 文件。
3. 按 package 分别发布到 npm（或使用 workspace publish 流程）。

更多实践请参见源码与 examples。欢迎把你遇到的问题写进 `docs/GUIDES.md` 并提交 PR。
