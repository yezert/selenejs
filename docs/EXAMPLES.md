# Examples / 示例

此文档列出仓库中可运行的示例并提供跳转与说明。

---

## basic
- 路径：[examples/basic/index.html](examples/basic/index.html)
- 说明（中文）：该示例通过 `index.html` 中的 `<script type="module" src="/src/main.js">` 直接加载源码，便于开发时热改与调试。主要入口：[examples/basic/src/main.js](examples/basic/src/main.js)
- Note (English): The basic example loads unbundled source directly. Serve the `examples/basic` folder with a static server and open the page to run the example.

```bash
cd examples/basic
python3 -m http.server 8000
# open http://localhost:8000
```

## 2026.html
- 路径：[examples/2026.html](examples/2026.html)
- 说明：演示或文档页面（根据仓库历史可能为展示用途），请直接打开该文件查看具体内容。

## 更多示例与约定

- 若要新增示例，请在 `examples/` 下创建子目录，包含 `index.html` 与 `src/`（若使用模块源码）；在 `docs/EXAMPLES.md` 中添加条目。

- 约定：示例目录建议包含：
	- `index.html`：入口页面
	- `src/`：示例源码（可选）
	- `README.md`：针对该示例的使用说明（建议双语）

### Example: how to switch to `dist` (说明)

如果你要用构建后的 `dist` 运行示例：

1. 构建 runtime / packages，确保 `dist/` 可用：

```bash
pnpm -w build
```

2. 把 `examples/basic/index.html` 中的 script 指向 `dist`：

```html
<script type="module" src="/dist/packages/core/index.js"></script>
```

（路径视构建输出而定，可能需要把 `dist` 内容复制到示例目录或调整静态服务器根路径）

---

更多示例：请检查 `examples/` 目录下的其它子目录或文件，若要新增示例，请在 `examples/` 下添加子目录并包含 `index.html` 与 `src/`（若需要）。
