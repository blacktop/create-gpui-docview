# create-gpui-docview Generator

This is a **CLI generator tool** that scaffolds GPUI document-view applications.

## Project Purpose

This tool generates complete GPUI workspace projects with:
- Pane/PaneGroup/TabBar system for document management
- Docking system with Panel trait for sidebars
- Example panel implementations (FileTree, Settings, Terminal)
- Modal components (Picker, Dialog)
- Theme management system

## Repository Structure

```
create-gpui-docview/
├── src/
│   └── main.rs              # CLI tool (uses clap, include_dir)
├── templates/
│   └── docview/             # Template for generated projects
│       ├── _Cargo.toml      # → Cargo.toml in output
│       ├── _AGENTS.md       # → AGENTS.md in output (for generated projects)
│       ├── README.md        # Template README
│       └── crates/          # All the GPUI components
│           ├── PROJECT_NAME/
│           ├── pane/
│           ├── docking/
│           ├── panels/
│           ├── modals/
│           └── theme/
├── Cargo.toml               # Generator binary manifest
└── AGENTS.md                # This file (for working on the generator)
```

## How It Works

1. **Template Embedding**: Uses `include_dir!` macro to embed `templates/docview/` at compile time
2. **File Copying**: Recursively copies template files to user-specified directory
3. **Name Replacement**: Replaces all instances of `PROJECT_NAME` with actual project name
4. **File Renaming**:
   - `_Cargo.toml` → `Cargo.toml`
   - `_AGENTS.md` → `AGENTS.md`
   - `PROJECT_NAME/` directories → actual project name

## Working on This Generator

### Adding New Template Files

1. Add files to `templates/docview/`
2. Use `PROJECT_NAME` placeholder where the project name should appear
3. Prefix with `_` if the file needs renaming during generation
4. Rebuild with `cargo build --release`

### Updating the Template

The template in `templates/docview/` represents what users will get when they run:
```sh
create-gpui-docview --name my-app
```

### Template Hygiene (avoid giant binaries)

- **Never commit `target/` or other build artifacts** anywhere under `templates/`. The generator embeds this tree via `include_dir!`, so stray build products explode the binary and can even break macOS linking (we just hit this on 26.2 beta).
- If you want to keep a reference workspace checked in (e.g. `templates/docview/gpui-docview-app/`), ensure a `.gitignore` inside that tree ignores `target/`, `Cargo.lock`, and any other generated files.
- When in doubt, run `git status -sb templates/docview` before committing; it should show only source files.

### Testing

```sh
# Build the generator
cargo build --release

# Test generation
cd /tmp
./target/release/create-gpui-docview --name test-project
cd test-project
cargo run -p test-project
```

### Change Verification Policy

- After **every** template or generator code change, run `make check` to ensure the generated workspace still builds (at least via `cargo check`).
- To keep terminal output tidy, pipe the `make check` output to a log if desired, e.g.  
  `make check > /tmp/create-gpui-docview.make-check.log`
- Review the log for warnings/errors and fix them before committing.

## References

### This Generator

- **Reference Implementation**: https://github.com/zed-industries/create-gpui-app
- **Clap Documentation**: https://docs.rs/clap/latest/clap/
- **include_dir**: https://docs.rs/include_dir/latest/include_dir/

### GPUI (What Generated Projects Use)

- **GPUI Crate**: https://crates.io/crates/gpui
- **GPUI API Docs**: https://docs.rs/gpui/latest/gpui/
- **GPUI Context7**: https://context7.com/websites/rs_gpui_gpui
- **Zed Source**: https://github.com/zed-industries/zed/tree/main/crates/gpui
- **GPUI Components**: https://github.com/longbridge/gpui-component - Excellent resource for learning how to build components with GPUI
  - Context7 access: https://context7.com/websites/longbridge_github_io_gpui-component

## Action Dispatch Pattern

The template uses a two-level action dispatch pattern for menu items and keyboard shortcuts:

### Problem
Menu items and keyboard shortcuts are registered at the **app level**, but action handlers are typically attached to entities via `.on_action()` in their render methods. When an action is triggered from a menu or keyboard shortcut, GPUI needs to know which window/entity to dispatch to.

### Solution
The template uses a hybrid approach:

1. **App-level handlers** - Registered in `main()` to catch actions from menus/keyboard
   ```rust
   app.on_action(|action: &ToggleSettings, cx| {
       if let Some(window) = cx.active_window() {
           window.update(cx, |_, window, cx| {
               window.dispatch_action(Box::new(action.clone()), cx);
           }).ok();
       }
   });
   ```

2. **Entity-level handlers** - Attached in `render()` to handle the dispatched actions
   ```rust
   v_flex()
       .on_action(cx.listener(Self::on_toggle_settings))
   ```

### How It Works
1. User clicks menu item or presses keyboard shortcut
2. App-level handler catches the action
3. Handler finds the active window via `cx.active_window()`
4. Handler dispatches the action to the window via `window.dispatch_action()`
5. Window dispatches to the focused entity (AppView in our case)
6. Entity's `.on_action()` handler receives and processes the action

### Key Points
- Each window handles its own actions independently
- Actions only fire once per gesture (no duplicate handling)
- Pattern follows Zed's approach (see `workspace::init()` in Zed source)
- For truly global actions like `Quit`, handle directly in app-level handler without dispatching to window
