# PROJECT_NAME

A minimal GPUI document-view application with:
- Left: a resizable file tree
- Right: a simple document viewer
- Clicking a file loads its contents into the viewer

## Structure

- `crates/PROJECT_NAME/` - Main application (file tree + document viewer)
- `crates/theme/` - Theme and settings management

This template also ships additional crates (`pane`, `docking`, `panels`, `modals`) as references for future expansion, but the app does not depend on them.

## Run

```sh
cargo run -p PROJECT_NAME
```

## Resources

- [GPUI Documentation](https://www.gpui.rs/)
- [Zed Editor](https://zed.dev/)
