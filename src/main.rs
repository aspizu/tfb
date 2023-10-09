use anyhow::Result;
use std::path::Path;

fn main() -> Result<()> {
    let path = Path::new(".");
    for entry in path.read_dir()? {
        let dir = entry?;
        let name = dir.file_name().into_string().unwrap();
        let meta = dir.metadata()?;
        let icon = char::from_u32(if meta.is_dir() {
            if dir.path().read_dir()?.count() == 0 {
                0xe5fe
            } else {
                0xf024b
            }
        } else if let Some(suffix) = name.rsplit_once(".").map(|v| v.1) {
            match suffix {
                "gitignore" => 0xe702,
                "toml" => 0xe615,
                "nix" => 0xf1105,
                "rs" => 0xe7a8,
                _ => 0xf15b,
            }
        } else {
            0xf15b
        })
        .unwrap();
        println!("{} {}", icon, name);
    }
    Ok(())
}
