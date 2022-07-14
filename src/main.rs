use std::ffi::{CStr, CString};

use nix::unistd;

fn main() {
    use nix::{
        sys::wait::waitpid,
        unistd::{fork, write, ForkResult},
    };

    match unsafe { fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            println!(
                "Continuing execution in parent process, new child has pid: {}",
                child
            );
            waitpid(child, None).unwrap();
        }
        Ok(ForkResult::Child) => {
            let command: CString = CString::new("./test_apps/ls.sh").unwrap();
            let arg: CString = CString::new("ls").unwrap();
            dbg!(unistd::execv(&command, &[arg]));
            unsafe { libc::_exit(0) };
        }
        Err(_) => println!("Fork failed"),
    }
}
