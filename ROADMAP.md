# 🗺️ ClipLumina Roadmap

This document outlines the evolutionary journey of ClipLumina from a simple clipboard watcher to an AI-powered productivity hub.

---

## 🟢 Stage 1: MVP (Current)

_Focus: Core functionality and "Mac-first" feel._

- [x] **Rust Clipboard Listener**: High-performance, low-latency background polling.
- [x] **System Tray Residency**: Accessory-mode window that hides/shows from the menu bar.
- [x] **Auto-Hide on Blur**: Window dismisses automatically when focus is lost.
- [x] **Basic Image Support**: Detect and preview images in history.
- [x] **Local Persistence**: History saved locally via `tauri-plugin-store`.
- [x] **Visual Feedback**: "Copied!" indicators and smooth transitions.
- [x] **Auto-Paste**: Simulated keystroke to paste content immediately upon selection.
- [] **Hex Color Detection**: Detect and preview hex colors in history.

---

## 🟡 Stage 2: Stable & Pro UI (Next Release)

_Focus: Reliability, Professional features, and Monetization._

- [ ] **Advanced Organization**:
  - **Pinned Items**: Enhanced management of permanent clips.
  - **Smart Folders**: Auto-categorize by Link, Code, Image, or Hex Color.
  - **Fuzzy Search**: Search through thousands of clips instantly.
- [ ] **Pro UI Hardening**:
  - **Theme Engine**: Support for different accent colors and glassmorphism levels.
  - **Settings Dashboard**: Configure history limits, shortcuts, and auto-paste behaviors.
- [ ] **Monetization Layer**:
  - **License Verification**: Implementation of a local-validity check for a Lifetime License.
  - **Pro Gating**: Move unlimited history and smart folders behind the Pro tier.
- [ ] **App Notarization**: Official Apple Developer signing for a smooth install experience.

---

## 🔴 Stage 3: Lumina AI (Later Stage)

_Focus: Intelligence and Cloud Connectivity._

- [ ] **Lumina AI Brain**:
  - **AI Refactor**: One-click code fixing or style rewriting for text.
  - **Smart Summarize**: Instant 3-bullet summaries for long copied articles.
  - **OCR (Optical Character Recognition)**: Extract text from copied screenshots locally.
- [ ] **Cloud Sync Engine**:
  - **End-to-End Encryption**: Sync clipboard between multiple Macs securely.
  - **Mobile Companions**: Minimalist iOS/Android apps to access your Mac history.
- [ ] **Subscription Model**:
  - Implementation of recurring billing to cover ongoing AI token costs.

---

## 🏗 Build Status

- **Current Version**: v0.1.0 (MVP)
- **Next Milestone**: v1.0.0 (Stable & Pro UI)
