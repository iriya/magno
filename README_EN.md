# 🔍 Magno

---

[简体中文](https://github.com/iriya/magno/master/README.md) | [English](https://github.com/iriya/magno/master/README_EN.md)

**Magno** is a lightweight, exquisite, secure, and distribution-friendly desktop word-lookup translation assistant built with `Tauri 2, Rust, and Vue 3`. It aims to provide a seamless and fast translation experience while protecting user API keys with system-level security.

### ✨ Core Features

* **🚀 Lightweight & Fast**: Powered by Tauri 2 architecture with ultra-low resource usage and instant startup.
* **📋 Auto Clipboard Monitoring**: Quietly monitors in the background, automatically capturing and pushing clipboard contents to the floating window.
* **🌐 Google Translation**: Seamlessly integrates Google Cloud Translation API (Basic v2) for accurate and efficient translations.
* **🔒 OS-Level Secure Storage**: Uses keyring to host the Google API Key securely in the OS credential manager (Windows Credential Manager / macOS Keychain), avoiding plain-text leakage.
* **🎨 Modern UI**: Built with Vue 3 and Tailwind CSS, featuring frosted glass effects, dual-column layout (original vs. translation), and smooth window dragging.
* **🛠️ System Tray Integration**: Supports dynamic tray icons, right-click menus, tooltips, and quick-access controls.

### 🛠️ Tech Stack

* **Core Framework**: Tauri 2 (Rust + Webview)
* **Frontend**: Vue 3, Tailwind CSS, Vite
* **System Hooks**: rdev / enigo, keyring
* **Network**: reqwest (Async HTTP Client)

### 🚀 Quick Start

Make sure you have Node.js, Rust, and the prerequisites for Tauri 2 installed.

**Clone the repository**
```Bash
git clone https://github.com/iriya/magno.git
cd magno
```

**Install frontend dependencies**
```Bash
npm install
```

**Run in development mode**
```Bash
npm run tauri dev
```

**Build for production**

```Bash
npm run tarui build
```