use std::{io, path::PathBuf, process::Command};

fn main() {
    while true {
        let paths = std::fs::read_dir("./apps/").unwrap();

        let paths: Vec<std::path::PathBuf> = paths.map(|p| p.unwrap().path()).collect();
        for (i, path) in paths.iter().enumerate() {
            println!("{}: Name: {}", i, path.display());
        }

        let mut index: usize;
        loop {
            if let Ok(i) = read_line("enter a game id:") {
                if i < paths.len() {
                    index = i;
                    break;
                }
            }
        }
        let chosen_path = paths[index].clone();
        start_app(chosen_path);
    }
}

fn start_app(chosen_path: std::path::PathBuf) -> io::Result<std::process::Child> {
    std::env::set_current_dir(chosen_path.clone());
    let paths = std::fs::read_dir("./").unwrap();
    let path_launch = std::fs::canonicalize("./").unwrap();
    let paths: Vec<std::path::PathBuf> = paths.map(|p| p.unwrap().path()).collect();
    let asset_root = path_launch.clone();
    for (i, path) in paths.iter().enumerate() {
        println!("{}: Name: {}", i, path.display());
        // FIXME: this still fails to load assets in bevy because of "current_exe" stil referring to Launcher https://github.com/bevyengine/bevy/pull/1801
        match Command::new(dbg!(path.display().to_string()))
            .env_remove("CARGO_MANIFEST_DIR")
            // Workaround over https://github.com/bevyengine/bevy/issues/5345
            .env("CARGO_MANIFEST_DIR", dbg!(asset_root.clone()))
            
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

fn read_line(arg: &str) -> Result<usize, ()> {
    use std::io::{stdin, stdout, Write};
    let mut s = String::new();
    print!("{}", arg);
    let _ = stdout().flush();
    stdin().read_line(&mut s).or(Err(()))?;
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    s.parse::<usize>().or(Err(()))
}
