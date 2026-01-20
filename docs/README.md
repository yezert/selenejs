# Docs

- 开发者：`DEVELOPERS.md`
- 用户：`USERS.md`
- 示例：`EXAMPLES.md`
- 安装：`INSTALLATION.md`
- API：`API.md`
- 贡献：`CONTRIBUTING.md`
- 指南：`GUIDES.md`
- 更新日志：`CHANGELOG.md`

**Examples / 示例**

- **Basic 示例（使用源码 / Uses Source files）**: 页面 [examples/basic/index.html](examples/basic/index.html) 通过以下方式直接加载示例源码：

	```html
	<script type="module" src="/src/main.js"></script>
	```

	因此 `examples/basic` 使用的是本地的 `src` 源码（未打包的开发版本），主要入口为 [examples/basic/src/main.js](examples/basic/src/main.js)。在开发或本地调试时直接打开该示例会运行源码而非已构建的 `dist` 包。

- **Basic (English)**: The page [examples/basic/index.html](examples/basic/index.html) loads the example source directly with:

	```html
	<script type="module" src="/src/main.js"></script>
	```

	Therefore `examples/basic` uses the local `src` files (the unbundled development version). The main entry is [examples/basic/src/main.js](examples/basic/src/main.js). When developing or debugging locally, the example runs from source instead of a built `dist` bundle.

---

更多示例说明请参见 `examples/` 目录下各子目录的 README 或直接打开对应的 HTML 文件。

