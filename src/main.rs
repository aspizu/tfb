use ansi_term::Colour::RGB;
use anyhow::Result;
use std::path::Path;

fn main() -> Result<()> {
    let path = Path::new(".");
    for entry in path.read_dir()? {
        // assign some data we'll need later into easily accessible variables
        let dir = entry?;
        let meta = dir.metadata()?;

        // color different types of files || WIP
        let name = dir.file_name().into_string().unwrap();

        // Find the icon
        // Check if the file is a directory
        let icon = char::from_u32(if meta.is_dir() {
            // check if directory contains files
            if dir.path().read_dir()?.count() == 0 {
                // icon =  closed folder icon
                0xcf07b
            } else {
                // icon = open folder icon
                0xf07c
            }
        // if the file has an extension, check if it is known
        } else if let Some(suffix) = name.rsplit_once(".").map(|v| v.1) {
            match suffix {
                // designate specific extensions to icons
                "gitignore" => 0xe702,
                "toml" => 0xe615,
                "nix" => 0xf1105,
                "rs" => 0xe7a8,
                // if it is unknown, make it a generic file icon
                _ => 0xf15b,
            }
        } else {
            // if all else fails, make it a generic file icon
            0xf15b
        })
        .unwrap();

        // color different types of files || WIP
        let mut name_colored = name.clone();
        let mut icon_colored = icon.clone().to_string();
        if meta.is_dir() {
            name_colored = RGB(20, 134, 227).paint(name).to_string();
            icon_colored = RGB(22, 148, 250).paint(icon.to_string()).to_string();
        }

        println!("{} {}", &icon_colored, &name_colored);
    }
    Ok(())
}
