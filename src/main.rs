use ansi_term::Color::*;
use std::path::{Path, PathBuf};

enum FileType {
    Directory,
    Other,
}

fn find_highlighting(path: PathBuf) -> String {
    let name_with_prefix = path.display().to_string();
    let name = name_with_prefix.strip_prefix("./").unwrap();
    let file_type: FileType = if path.is_dir() {
        FileType::Directory
    } else {
        FileType::Other
    };

    match file_type {
        FileType::Directory => return RGB(66, 135, 245).paint(name).to_string(),
        FileType::Other => return name.to_owned(),
    }
}

fn main() -> Result<(), anyhow::Error> {
    let paths = Path::new(".");
    for entry in paths.read_dir()? {
        let path = entry?.path();

        println!("{}", find_highlighting(path));
    }

    Ok(())
}
