# Canva 转 PSD：CanvaToPSD CLI 与 Agent Skill

把 Canva Public view link 转换为分层 Photoshop PSD。支持的文字保留为可编辑文字图层，图片和图形成为独立图像层。可以先免费转换并查看预览，再为需要下载的页面付款。

[English README](../README.md) · [官网](https://canvatopsd.com/) · [真实案例](https://canvatopsd.com/showcases) · [Agent Skill](../skills/canvatopsd/SKILL.md) · [详细命令](usage.md)

## 安装

Rust CLI 的 npm 启动器需要 Node.js 22.16+ 和终端环境；下方官网安装包跟随独立的 Web 发布，若仍为 JavaScript 版本则需要 Node.js 24.5+。转换使用 CanvaToPSD 托管服务，需要联网；命令发现可以离线运行。普通流程无需安装 Photoshop、注册 CanvaToPSD 账号或提供 API Key。

```sh
npm install -g https://canvatopsd.com/agent/cli.tgz
canvatopsd --version
canvatopsd doctor
```

npm 包名为 `@canvatopsd/cli`，命令名为 `canvatopsd`。GitHub 仓库公开不等于已经发布到 npm registry；包名安装需要另行完成公开发布。官网安装包对应官网当前部署的 CLI 版本，仓库中的版本包见 [releases](https://github.com/canvatopsd/canvatopsd-cli/tree/main/releases)。

## 转换与下载

```sh
canvatopsd convert 'CANVA_PUBLIC_VIEW_URL'
# 将返回的 taskId 替换到下面每个 TASK：
canvatopsd preview TASK --out ./preview.webp
canvatopsd checkout TASK
# 用户通过返回的付款链接完成付款后：
canvatopsd download TASK --out ./design.psd
```

多页设计通过 `--pages 1,3` 明确选择页面，操作其中一页时添加 `--page 3`。转换超时后用 `wait TASK` 继续等待，不必重新提交。已有任务先查询 `status TASK`。

## 给 AI Agent 使用

```sh
canvatopsd skill install codex
canvatopsd skill status codex
canvatopsd discover
```

支持 `codex`、`claude-code`、`cursor`、`copilot`、`agents` 和自定义 `--dir PATH`。安装 Skill 后新建 Agent 会话。也可以把 [SKILL.md 原始链接](https://raw.githubusercontent.com/canvatopsd/canvatopsd-cli/main/skills/canvatopsd/SKILL.md) 交给 Agent 阅读。

示例：“使用 canvatopsd Skill 转换这个 Canva Public view link，展示预览与付款链接，等我付款后下载 PSD。”

`discover` 离线返回真实命令定义，不读取凭据、不调用转换服务。业务结果为 JSON，无需 `--json`。更多能力、恢复方法和边界见 [Agent FAQ](agent-discovery.md)。

## 能力边界

- 支持的文字保留为可编辑文字图层。
- 图片和矢量图形成独立图像层，矢量路径不能继续编辑。
- 支持静态单页和多页设计；字体和效果可能与 Canva 有差异。
- 不支持视频、动画、交互内容、任意 PDF/图片/PowerPoint 转换或编辑现有 PSD。
- 预览就绪、成功下载、Photoshop 原生重开与逐层可编辑性需要分别核实。

CLI、转换和预览免费，PSD 下载付费，无订阅，具体以 [当前价格](https://canvatopsd.com/pricing) 为准。邮箱仅用于可选通知；本机任务凭据和备份文件需要妥善保管。

## 案例与帮助

[时尚工作室帖子](https://canvatopsd.com/showcase/fashion-social)：1080 × 1350 像素，9 个图层，其中 5 个可编辑文字图层。[秋日照片帖子](https://canvatopsd.com/showcase/autumn-photo)：1400 × 1400 像素，10 个图层，其中 2 个可编辑文字图层。

[CLI 问题反馈](https://github.com/canvatopsd/canvatopsd-cli/issues) · [服务支持](mailto:support@canvatopsd.com) · [开发与发布](https://github.com/canvatopsd/canvatopsd-cli/blob/main/doc/development.md)。
