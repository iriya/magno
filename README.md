# 🔍 Magno

---

[简体中文](https://github.com/iriya/magno/blob/master/README.md) | [English](https://github.com/iriya/magno/blob/master/README_EN.md)

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

### 🚀 快速上手 (开发者)

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

### 📖 使用指南

1. **配置 Google API Key**：
    * 启动程序后，点击系统托盘的放大镜图标呼出悬浮窗，或者点击悬浮窗右上角的设置图标（⚙️）。
    * 在设置面板中填入你的 **Google Cloud Translation API (Basic v2) 密钥**，点击“保存”。API Key 会被安全加密存储在系统底层的凭据管理器中（如 Windows 凭据管理器或 macOS 钥匙串）。

2. **划词翻译**：
    * 在任意软件或网页中，**按住键盘上的 `Ctrl` 键并选中（划取）** 你需要翻译的文本。
    * 程序会静默捕获剪贴板内容，自动请求谷歌翻译并在鼠标附近（或指定位置）弹出悬浮窗，以**双栏卡片**的形式清晰展示“原文”与“译文”。

3. **系统托盘操作**：
    * 右键点击托盘图标可以呼出菜单，支持快速唤醒悬浮窗或退出程序。