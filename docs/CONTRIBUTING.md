# Contributing / 贡献指南

欢迎贡献！本节简要说明如何为 SeleneJS 提交 issue、PR 以及参与讨论（中/EN）。

## 提交 Issue / Submitting issues
- 请在 GitHub 仓库的 Issues 里创建问题，描述复现步骤、期望行为与实际行为，并附上最小可复现的示例。

## 提交 PR / Pull Requests

1. Fork 仓库并新建分支：`git checkout -b feat/your-feature`
2. 在本地开发并运行测试（若有）：`pnpm install && pnpm -w build`
3. 提交并开 PR，PR 描述应包含变更概述、动机和兼容性说明。

## 代码风格与约定
- 使用 Prettier/ESLint（仓库若包含配置，请遵循）；提交前运行 `pnpm -w test` 或 `pnpm -w lint`（若存在）。

## 本地测试与 CI
- 本项目采用 GitHub Actions（若存在）进行 CI，PR 会自动触发构建与测试流程。若 CI 失败，请在本地复现并修复再提交。

## 翻译与文档贡献
- 文档建议双语（中文/英文），为便于维护请在 PR 描述中说明该文档的用途与目标读者。

谢谢你的贡献！
