use ansi_term::Colour::RGB;
use anyhow::Result;
use std::{fs::DirEntry, path::Path};

fn find_display_properties(item: DirEntry) -> Result<(String, String), anyhow::Error> {
    let mut name = item.file_name().into_string().unwrap();
    let meta = item.metadata()?;
    let suffix = name.rsplit_once(".").map(|v| v.1);
    let mut icon = "";

    if meta.is_dir() {
        name = RGB(20, 134, 227).paint(name).to_string();
        if item.path().read_dir().count() == 0 {
            icon = RGB(20, 134, 227).paint("󰉋").to_string().as_str();
        } else {
            icon = RGB(20, 134, 227).paint("󰝰").to_string().as_str();
        }
    } else {
        match suffix {
            "gitignore" => {
                icon = "" /* name = RGB(255, 255, 255).paint(name).to_string() */
            }
            "toml" => icon = "",
            "nix" => icon = "󱄅",
            "rs" => icon = "",
            _ => icon = "",
        }
    }

    (icon.to_string(), name)
}
// Can possibly error, so we return a result.
// Hopefully `Ok(())` :>
fn main() -> Result<()> {
    // todo: grab optional input to list another directory

    // make a new path struct and loop through all of the
    // entries inside
    let path = Path::new(".");
    for entry in path.read_dir()? {
        let dir = entry?;

        //let (icon, name) = find_display_properties();

        //println!("{} {}", icon, name);
    }
    Ok(())
}
