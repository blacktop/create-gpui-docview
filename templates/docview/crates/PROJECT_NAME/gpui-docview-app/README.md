# gpui-docview-app

A full-featured GPUI document-view application with a complete UI system.

## Features

### 🎯 Core UI Components

- **Menu Bar**: Full menu system with File, Edit, View, Window, and Help menus
- **File Tree Sidebar**: Collapsible, resizable file browser with lazy-loading
- **Document Tabs**: Multi-tab document viewing with close buttons and dirty indicators
- **Status Bar**: Shows file info, cursor position, line count, encoding, and line endings
- **Settings Modal**: Beautiful overlay settings panel with toggles and preferences

### ⌨️ Keyboard Shortcuts

All menu items have keyboard shortcuts:

**File Menu:**
- `cmd+n` - New File
- `cmd+o` - Open File
- `cmd+s` - Save
- `cmd+shift+s` - Save As
- `cmd+w` - Close Tab
- `cmd+q` - Quit

**Edit Menu:**
- `cmd+z` - Undo
- `cmd+shift+z` - Redo
- `cmd+x` - Cut
- `cmd+c` - Copy
- `cmd+v` - Paste
- `cmd+f` - Find
- `cmd+shift+f` - Replace

**View Menu:**
- `cmd++` - Zoom In
- `cmd+-` - Zoom Out
- `cmd+0` - Reset Zoom
- `cmd+b` - Toggle Sidebar
- `cmd+j` - Toggle Footer
- `cmd+,` - Toggle Settings
- `ctrl+cmd+f` - Toggle Fullscreen

**Window Menu:**
- `cmd+shift+n` - New Window
- `cmd+m` - Minimize
- `cmd+\` - Split Vertical
- `cmd+shift+\` - Split Horizontal

### 🎨 Layout Features

- **Resizable Sidebar**: Drag the handle between sidebar and editor to resize
- **Split Panes**: Support for vertical and horizontal document splits (coming soon)
- **Scrollable Content**: Both sidebar and editor have proper scrollbars
- **Responsive Design**: Clean, modern interface that adapts to your content

### 🔧 Architecture

The project is organized as a workspace with modular crates:

```
crates/
├── gpui-docview-app/    # Main application
├── menubar/         # Menu bar component
├── keybinds/        # Keyboard shortcut system
├── statusbar/       # Footer/status bar
├── pane/            # Tab and pane management
├── docking/         # Panel docking system
├── panels/          # File tree, settings, terminal panels
├── modals/          # Overlays (picker, dialog, settings)
└── theme/           # Theme and color system
```

## Getting Started

### Build and Run

```sh
cargo run -p gpui-docview-app
```

### Development

```sh
# Check for errors
cargo check

# Run with logging
RUST_LOG=info cargo run -p gpui-docview-app

# Build release version
cargo build --release -p gpui-docview-app
```

## Customization

### Adding Menu Items

Edit `crates/menubar/src/menu_bar.rs` to add new menu items and actions.

### Modifying Themes

Edit `crates/theme/src/theme.rs` to customize colors and spacing.

### Adding Panels

Implement the `Panel` trait in `crates/docking/src/panel_trait.rs` for new panels.

### Extending Key Bindings

Add new shortcuts in `crates/keybinds/src/lib.rs`.

## License

Apache-2.0
