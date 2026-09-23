# 🔍 Magno

---

[简体中文](https://github.com/iriya/magno/blob/master/README.md) | [English](https://github.com/iriya/magno/blob/master/README_EN.md)

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

### 🚀 Quick Start (for Developers)

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

### 📖 User Guide

1. **Configure Google API Key**:
    * After launching the application, click the magnifying glass icon in the system tray to bring up the floating window, or click the settings icon (⚙️) in the upper right corner of the window.
    * Enter your **Google Cloud Translation API (Basic v2) Key** in the settings panel and click "Save". The API Key will be securely and encrypted stored in the underlying system credential manager (such as Windows Credential Manager or macOS Keychain).

2. **Word-Lookup Translation**:
    * In any software or webpage, **hold down the `Ctrl` key on your keyboard and select (highlight)** the text you want to translate.
    * The application will silently capture the clipboard content, automatically request a Google translation, and pop up a floating window near the mouse (or at a designated position), clearly displaying both the "Original" and "Translation" in a **dual-column card layout**.

3. **System Tray Operations**:
    * Right-click the tray icon to open the menu, which supports quickly waking up the floating window or exiting the application.