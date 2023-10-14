use ansi_term::Color::*;
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

enum FileType {
    Directory,
    File,
    Symlink,
}

impl FileType {
    fn from_path(path: &PathBuf) -> FileType {
        // proof of concept

        if path.is_dir() {
            return FileType::Directory;
        } else if path.is_file() {
            return FileType::File;
        } else if path.is_symlink() {
            return FileType::Symlink;
        } else {
            return FileType::File;
        }
    }
}

fn find_highlighting(file_type: &FileType, name: &str) -> String {
    match file_type {
        FileType::Directory => return RGB(66, 135, 245).paint(name).to_string(),
        FileType::File => return name.to_owned(),
        FileType::Symlink => return RGB(176, 176, 62).paint(name).to_string(),
    }
}

fn find_icon(path: &Path) -> &str {
    if path.is_dir() {
        return "󰉋";
    };
    let extension = path
        .extension()
        .unwrap_or(OsStr::new(""))
        .to_str()
        .unwrap_or("");
    match extension {
        "" => "󰈔",
        &_ => "󰈔",
    }
}

fn main() -> Result<(), anyhow::Error> {
    let paths = Path::new(".");
    for entry in paths.read_dir()? {
        let path = entry?.path();

        let icon = find_icon(&path);
        let name = path.display().to_string();
        let name = name.strip_prefix("./").unwrap();
        let listing = format!("{} {}", icon, name);
        let filetype = FileType::from_path(&path);

        let listing_highlighted = find_highlighting(&filetype, &listing);

        println!("{}", listing_highlighted);
    }

    Ok(())
}
