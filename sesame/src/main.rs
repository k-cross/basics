use libc::{O_RDONLY, c_void, size_t};
use std::ffi::CString;
use std::fs::File;

fn main() {
    // using libc syscalls to write a msg to stdout
    let msg = "Hello, libc syscalls!\n";
    let stdout_fd = 1;
    unsafe {
        libc::write(
            stdout_fd,
            msg.as_ptr() as *const c_void,
            msg.len() as size_t,
        );
    }

    let filename = CString::new("example.txt").unwrap();

    unsafe {
        // Tell the OS to open a new file which gives us a FD
        let fd = libc::open(filename.as_ptr(), O_RDONLY);
        if fd == -1 {
            eprintln!("Failed to open file.");
            return;
        }

        let mut buffer = [0u8; 100];
        let bytes_read = libc::read(fd, buffer.as_mut_ptr() as *mut _, buffer.len());

        println!("Bytes read: {:?}", &bytes_read);

        if bytes_read > 0 {
            let output = String::from_utf8_lossy(&buffer[..bytes_read as usize]);
            println!("File content:\n{}", output);
        } else {
            eprintln!("Failed to read file.");
        }

        // Tell the OS that we're done with the FD
        let close_val = libc::close(fd);
        println!("fd = {}, close = {}", fd, close_val);
    }

    // Regular rust file procedure
    let file = File::open("/Users/ken/src/hello.d").unwrap();
    println!("This is a standard file: {:?}", file);
}
