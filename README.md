# Tauri + React + Typescript

This template should help get you started developing with Tauri, React and Typescript in Vite.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

# 💡 ClipLumina

**ClipLumina** is not just another clipboard manager; it's a high-performance, AI-integrated productivity companion built with **Tauri 2.0** and **Rust**. Designed to be lightweight, secure, and future-proof.

---

## 🚀 Why ClipLumina?

Most clipboard tools are "Passive"—they just store text. **ClipLumina** is "Active." It understands what you copy and helps you take the next step instantly, while keeping your Mac's RAM happy.

---

## 🛠 MVP Features (Phase 1)

_Current focus: The core experience._

- **Ultra-Light Background Listener**: Written in pure Rust to monitor your clipboard with near-zero CPU usage.
- **System Tray Residency**: Lives in your macOS menu bar—no bulky windows needed.
- **Live History List**: Real-time React UI updates the moment you press `Cmd+C`.
- **Smart Truncation**: Clean UI that handles long strings and code snippets gracefully.
- **Local Persistence**: Your history stays on your machine. Privacy by design.

---

## 🔮 Future Releases & Pro Features

Check out our [**Pro Features Vision**](./PRO_FEATURES.md) for a detailed look at where ClipLumina is heading.

### Phase 2: The "Lumina" Brain (AI Integration)

- **AI Polish**: One-click to fix grammar or refactor code snippets directly from the history.
- **Smart Categorization**: Automatically detects if a clip is a URL, Color Code, SSH Key, or Snippet.
- **Instant Summary**: Copy a long article? Get a 3-bullet point summary in the tray.

### Phase 3: Cross-Platform Sync (Tauri 2.0 Power)

- **iOS & Android Companions**: Use Tauri 2.0's mobile support to sync your clipboard between your Mac and your phone.
- **End-to-End Encryption**: Syncing data safely without us ever seeing it.

### Phase 4: Developer Tools

- **JSON Formatter**: Previews and formats minified JSON automatically.
- **Color Picker**: Recognizes HEX/RGB codes and shows a visual preview.

---

## 🏗 Tech Stack

- **Core**: [Tauri 2.0](https://tauri.app/) (Rust)
- **Frontend**: React + TypeScript
- **Styling**: Tailwind CSS
- **State Management**: React Hooks + Tauri Events

---

## 📦 Getting Started

1. **Clone the repo:**
   ```bash
   git clone [https://github.com/your-username/clip-lumina.git](https://github.com/your-username/clip-lumina.git)
   ```
