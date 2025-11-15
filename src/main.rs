use clap::Parser;
use include_dir::{include_dir, Dir, DirEntry};
use std::{
    fs,
    io::Result,
    path::{Path, PathBuf},
};

static TEMPLATE_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates/docview");
static DEFAULT_PROJECT_NAME: &str = "gpui-docview-app";

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    /// Name of the new project
    #[clap(short, long, default_value = DEFAULT_PROJECT_NAME, value_parser = parse_name)]
    name: Option<String>,
}

fn parse_name(name: &str) -> Result<String> {
    if name.is_empty() {
        Ok(DEFAULT_PROJECT_NAME.to_string())
    } else {
        Ok(name.to_string())
    }
}

fn copy_and_replace(
    destination_path: &mut PathBuf,
    project_name: &str,
    source_dir: &Dir,
) -> Result<()> {
    const WORD_TO_REPLACE: &str = "PROJECT_NAME";
    if destination_path.file_name().unwrap() == WORD_TO_REPLACE {
        destination_path.set_file_name(project_name)
    };

    fs::create_dir_all(&destination_path)?;
    for entry in source_dir.entries() {
        let relative_path = entry.path().strip_prefix(source_dir.path()).unwrap();
        let mut entry_path = destination_path.to_owned().join(relative_path);
        match entry {
            DirEntry::Dir(dir) => copy_and_replace(&mut entry_path, project_name, dir)?,
            DirEntry::File(file) => {
                if let Some(content) = file.contents_utf8() {
                    let mut content_string = content.to_string();
                    match file.path() {
                        path if path.file_name().unwrap() == "_Cargo.toml" => {
                            entry_path.set_file_name("Cargo.toml")
                        }
                        path if path.file_name().unwrap() == "_AGENTS.md" => {
                            entry_path.set_file_name("AGENTS.md")
                        }
                        _ => {}
                    }
                    content_string = content_string.replace(WORD_TO_REPLACE, project_name);
                    fs::write(&entry_path, content_string)?;
                }
            }
        }
    }
    Ok(())
}

/// Create a new GPUI document-view application
fn main() -> Result<()> {
    let args = Args::parse();

    let project_name = args.name.unwrap();
    let mut project_path = Path::new(&project_name).to_owned();

    if project_path.exists() {
        println!("'{}' already exists.", project_name);
        return Ok(());
    }

    copy_and_replace(&mut project_path, &project_name, &TEMPLATE_DIR)?;

    println!(
        "Successfully created new GPUI docview app '{}'",
        project_name
    );

    Ok(())
}
