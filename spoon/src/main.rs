use libc::{c_char, execve, fork, waitpid};
use std::ffi::CString;

fn main() {
    // fork
    unsafe {
        // fork the current process at this point
        let pid = fork();
        if pid == 0 {
            println!("Child process!");

            // execve
            let path = CString::new("/bin/ls").unwrap();
            let arg1 = CString::new("-l").unwrap();
            let args = [path.as_ptr(), arg1.as_ptr(), std::ptr::null()];
            let env = [std::ptr::null::<c_char>()];

            let exit_code = execve(path.as_ptr(), args.as_ptr(), env.as_ptr());
            // Will never execute this because the program stack in the child is
            // rewritten for the program replacing it which is `ls -l` here.
            println!("Child exit status: {}", exit_code);
        } else {
            println!("Parent process! Child PID: {}", pid);
            let mut status: i32 = 0;
            waitpid(pid, &mut status as *mut i32, 0);
            println!("Child PID: {}, Exit Status: {}", pid, status);
        }
    }
}
