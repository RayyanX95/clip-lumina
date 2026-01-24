# Clip Lumina MVP Implementation Plan

This document outlines the step-by-step plan to build the Minimum Viable Product (MVP) for Clip Lumina.

## Phase 1: Core Backend (Rust)
-   [x] **Step 1.1: Implement Background Clipboard Watcher**
    -   Modify `src-tauri/src/main.rs`.
    -   Spawn a dedicated thread that polls the clipboard.
    -   Use `tauri::Emitter` to emit an event (`clipboard://change`) to the frontend whenever content changes.
    -   *Goal:* Frontend should automatically know when clipboard changes without clicking a button.
-   [x] **Step 1.2: System Tray Setup**
    -   Add `tauri-plugin-positioner` (optional but good for tray apps) or standard Tauri tray configuration.
    -   Modify `src-tauri/tauri.conf.json` (or capabilities) if necessary for tray permissions.
    -   Update `main.rs` to initialize the System Tray.
    -   Configure the window to toggle visibility when the tray icon is clicked.
    -   hide the dock icon (optional for "pure" tray apps, but good for MVP).

## Phase 2: Frontend Foundation (React + Tailwind)
-   [x] **Step 2.1: State Management for History**
    -   Create a `useClipboardHistory` hook.
    -   Listen for the `clipboard://change` event from Tauri.
    -   Maintain a list of recent clips (e.g., last 50 items).
-   [x] **Step 2.2: History List UI**
    -   Create a `HistoryItem` component.
    -   Design the list view with "Smart Truncation" (CSS `text-overflow: ellipsis`).
    -   Apply the "Premium" aesthetic (glassmorphism, nice typography).

## Phase 3: Data Persistence
-   [x] **Step 3.1: Local Storage Strategy**
    -   Decide on storage: `tauri-plugin-store` (simple key-value) or File I/O.
    -   For MVP, `tauri-plugin-store` is easiest and robust enough.
-   [x] **Step 3.2: Implement Load/Save**
    -   Load history on app launch.
    -   Save history whenever a new item is added.

## Phase 4: Refinement & Polish
-   [x] **Step 4.1: Window Behavior**
    -   Ensure standard window decorations are hidden (no title bar).
    -   Make the window appear "floating" near the tray or centered.
    -   "Lost Focus" behavior: Close/Hide window when user clicks away.
-   [x] **Step 4.2: Performance Tuning**
    -   Verify CPU usage of the polling thread. (Measured 0.0% usage in idle state via `ps aux`)

---

## Execution Log
*Check off items as we complete them.*
