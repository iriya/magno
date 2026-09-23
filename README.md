# 🔍 Magno

---

[简体中文](https://github.com/iriya/magno/master/README.md) | [English](https://github.com/iriya/magno/master/README_EN.md)

**Magno** 是一款基于 `Tauri 2 + Rust + Vue 3` 开发的轻量、精致、安全且易于分发的桌面端划词翻译助手。它旨在提供无感、快速的翻译体验，并通过系统级安全凭据保护用户的 API 密钥。

### ✨ 核心功能

* **🚀 轻量高效**：基于 Tauri 2 架构，资源占用极低，冷启动秒开。
* **📋 自动划词捕获**：后台静默监听，自动捕获剪贴板内容并推送到悬浮窗。
* **🌐 谷歌翻译支持**：无缝集成 Google Cloud Translation API (Basic v2)，翻译精准高效。
* **🔒 系统级安全存储**：采用 `keyring` 将 Google API Key 托管至操作系统凭据管理器（Windows 凭据管理器 / macOS 钥匙串），拒绝明文泄露。
* **🎨 精致现代 UI**：采用 Vue 3 + Tailwind CSS 构建，支持毛玻璃特效、双栏对照视图（原文与译文）及窗口自由拖拽。
* **🛠️ 系统托盘支持**：支持动态托盘图标、右键菜单、悬停提示及快速呼出。

### 🛠️ 技术栈

* **核心框架**: Tauri 2 (Rust + Webview)
* **前端界面**: Vue 3, Tailwind CSS, Vite
* **系统交互**: `rdev` / `enigo` (系统监听与控制), `keyring` (安全凭据)
* **网络请求**: `reqwest` (异步 HTTP 客户端)

### 🚀 快速上手

确保你的开发环境已安装 **Node.js**、**Rust** 以及 **Tauri 2** 所需的依赖。

**克隆项目**
```bash
git clone https://github.com/iriya/magno.git
cd magno
```

**安装前端依赖**
```bash
npm install
```

**启动开发模式**
```bash
npm run tauri dev
```

**打包发布**
```bash
npm run tarui build
```