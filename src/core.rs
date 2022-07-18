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
    let paths = std::fs::read_dir(&chosen_path).unwrap();
    let path_launch = std::fs::canonicalize(&chosen_path).unwrap();
    let paths: Vec<PathBuf> = paths.map(|p| p.unwrap().path()).collect();
    let asset_root = path_launch.clone();
    for (i, path) in paths.iter().enumerate() {
        println!("{}: Name: {}", i, path.display());
        match Command::new(path.display().to_string())
            // Workaround over https://github.com/bevyengine/bevy/issues/5345
            .env("CARGO_MANIFEST_DIR", &asset_root)
            //
            .env("BEVY_ASSET_ROOT", &asset_root)
            .current_dir(&path_launch)
            .spawn()
        {
            Ok(child) => {
                dbg!("great");
                return Ok(child);
            }
            Err(e) => dbg!(e),
        };
    }
    Err(io::Error::new(io::ErrorKind::Other, "Something went wrong"))
}
