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