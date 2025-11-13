# GPUI Docview Project Guide

This project uses GPUI, the GPU-accelerated Rust UI framework developed for the Zed code editor.

## Your Role as an Agent

### 1. Understand the Core Concepts

- A UI is composed of **Entities** managed by `AppContext`
- Layout uses **GPUI flexbox**
- Components implement `Render`
- Panels implement `Panel`
- Document views are assembled via `Pane` + `PaneGroup`
- Sidebar tools are assembled via `Dock` + `Panel`

### 2. When Adding or Modifying UI

- **Never directly mutate other components**; use events
  - Example: `emit(OpenFile(path))`
- Prefer **flexbox layouts** over absolute sizing
- Use **rem units** for all spacing & typography (scalable UI)
- Integrate with the global **Theme** for colors & fonts
- Ensure all widgets are **Focusable** if they need keyboard input

## File Locations

- `crates/PROJECT_NAME/`: Main application / workspace / window root
- `crates/pane/`: Document view models & rendering (Pane, PaneGroup, TabBar)
- `crates/docking/`: Dock + Panel architecture
- `crates/panels/`: Built-in panel implementations (FileTree, Settings, Terminal)
- `crates/modals/`: Modal / Picker components
- `crates/theme/`: Theming system and settings

## Patterns to Follow

- Use GPUI **builder syntax**: `div().child(...)`
- Use `.flex_row()` / `.flex_col()` for all major layout containers
- Always handle resizing via **draggable splitters** using pointer events
- All panels **must implement `Panel`**
- All document-like UI **must live in a `Pane`**

## References

### Official GPUI Documentation

- **GPUI Crate**: https://crates.io/crates/gpui
- **GPUI API Docs**: https://docs.rs/gpui/latest/gpui/
- **GPUI Context7 Docs**: https://context7.com/websites/rs_gpui_gpui

### Zed Source Code Examples

- **GPUI Source**: https://github.com/zed-industries/zed/tree/main/crates/gpui
- **GPUI Macros**: https://github.com/zed-industries/zed/tree/main/crates/gpui_macros
- **Workspace Architecture**: https://github.com/zed-industries/zed/tree/main/crates/workspace
- **UI Components**: https://github.com/zed-industries/zed/tree/main/crates/ui

### Blog Articles

- **GPUI Introduction**: https://zed.dev/blog/gpui
- **Ownership in GPUI**: https://zed.dev/blog/ownership