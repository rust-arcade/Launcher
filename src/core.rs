pub fn list_games() -> Vec<std::path::PathBuf> {
    let paths = std::fs::read_dir("./apps/").unwrap();
    let paths: Vec<std::path::PathBuf> = paths.map(|p| p.unwrap().path()).collect();
    for (i, path) in paths.iter().enumerate() {
        println!("{}: Name: {}", i, path.display());
    }
    paths
}

use std::{io, path::PathBuf, process::Command};

pub fn launch_app(chosen_path: PathBuf) -> io::Result<std::process::Child> {
    std::env::set_current_dir(chosen_path)?;
    let paths = std::fs::read_dir("./").unwrap();
    let path_launch = std::fs::canonicalize("./").unwrap();
    let paths: Vec<PathBuf> = paths.map(|p| p.unwrap().path()).collect();
    let asset_root = path_launch.clone();
    for (i, path) in paths.iter().enumerate() {
        println!("{}: Name: {}", i, path.display());
        match Command::new(dbg!(path.display().to_string()))
            // Workaround over https://github.com/bevyengine/bevy/issues/5345
            .env("CARGO_MANIFEST_DIR", dbg!(asset_root.clone()))
            //
            .env("BEVY_ASSET_ROOT", dbg!(asset_root.clone()))
            .current_dir(dbg!(path_launch.clone()))
            .spawn()
        {
            Ok(child) => {
                dbg!("great");
                return Ok(child);
            }
            Err(e) => dbg!(e),
        };
    }
    Err(io::Error::new(io::ErrorKind::Other, "fuck"))
}
