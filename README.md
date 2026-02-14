# Tauri v2 × Leptos – invoke / emit Sample

This repository contains minimal examples demonstrating how IPC works in a Tauri v2 + Leptos application.

It focuses on understanding the structure and flow of:

- `invoke` (UI → Native command)
- `emit` (Native → UI event)

Built with:

- Rust
- Tauri v2
- Leptos 0.8

---

## ✨ What This Example Does

### invoke example

- Takes two numbers from the UI
- Calls a Tauri command using `invoke`
- Displays the result returned from the native layer

Flow:

UI (Leptos WASM)

↓ invoke

Tauri Core

↓

Native Rust command

↓ Promise

UI updates Signal

---

### emit example

- Listens for window resize events
- Emits the window size from the native layer
- Updates the UI when the event is received

Flow:

Native Rust (window event)

↓ emit

Tauri event system

↓ listen

UI updates Signal


---

## 🧠 Architecture Overview

Tauri IPC can be understood as two directions:

| Direction | Mechanism | Concept |
|------------|------------|----------|
| UI → Native | `invoke` | Command (request + response) |
| Native → UI | `emit`   | Notification (event-driven) |

Understanding this distinction makes it easier to design application state flow.

---

## 🚀 Setup

Make sure you have:

- Rust (stable)
- Tauri CLI

Then run:

```bash
cargo tauri dev
```

## 📚 Related Article

This repository accompanies the following article:

## 👉 Zenn:
【Tauri v2 × Leptos】invoke, emitの仕組みを構造から理解する

## 📝 Notes

invoke returns a JavaScript Promise internally, which is converted to JsFuture on the Rust/WASM side.

emit uses Tauri’s event system and requires event name consistency between native and UI layers.

Closures used in WASM event listeners are forget()-ed to prevent them from being dropped by Rust.

## 🔍 Purpose

This repository is part of my exploration of Tauri + Leptos architecture and IPC design patterns.

More examples may be added in the future.