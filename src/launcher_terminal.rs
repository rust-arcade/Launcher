pub fn run() {
    loop {
        let paths = crate::core::list_games();

        let index: usize;
        loop {
            if let Ok(i) = read_line("enter a game id:") {
                if i < paths.len() {
                    index = i;
                    break;
                }
            }
        }
        let chosen_path = paths[index].clone();
        if let Ok(mut child) = crate::core::launch_app(chosen_path) {
            child.wait();
        };
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
