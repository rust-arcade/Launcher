use std::{
    error::Error,
    ffi::{CStr, CString},
    io,
};

use nix::unistd;

fn main() {
    use nix::{
        sys::wait::waitpid,
        unistd::{fork, write, ForkResult},
    };

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
        println!("{}: Name: {}", index, chosen_path.display());
        match unsafe { fork() } {
            Ok(ForkResult::Parent { child, .. }) => {
                println!(
                    "Continuing execution in parent process, new child has pid: {}",
                    child
                );
                waitpid(child, None).unwrap();
            }
            Ok(ForkResult::Child) => {
                start_app(chosen_path);

                unsafe { libc::_exit(0) };
            }
            Err(_) => println!("Fork failed"),
        }
    }
}

fn start_app(chosen_path: std::path::PathBuf) {
    std::env::set_current_dir(chosen_path);
    let paths = std::fs::read_dir("./").unwrap();

    let paths: Vec<std::path::PathBuf> = paths.map(|p| p.unwrap().path()).collect();
    for (i, path) in paths.iter().enumerate() {
        println!("{}: Name: {}", i, path.display());
        let command_string = path.display().to_string();
        let command: CString = CString::new(command_string.clone()).unwrap();
        let arg: CString = CString::new(command_string).unwrap();
        std::env::set_var("PWD", dbg!(std::env::current_dir().unwrap()));
        let env: Vec<CString> = std::env::vars()
            .map(|(k, v)| CString::new(format!("{k}={v}")).unwrap())
            .collect();
        dbg!(unistd::execve(&dbg!(command), &[arg], &dbg!(env)));
    }
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
