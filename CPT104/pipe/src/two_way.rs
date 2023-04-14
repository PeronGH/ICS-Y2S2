use nix::sys::wait::wait;
use nix::unistd::{close, fork, pipe, read, write, ForkResult};
use std::ffi::CString;
use std::process::exit;

pub fn main() -> nix::Result<()> {
    // Create 2 pipes
    let (pipe1_read_fd, pipe1_write_fd) = pipe()?;
    let (pipe2_read_fd, pipe2_write_fd) = pipe()?;

    match unsafe { fork() }? {
        ForkResult::Parent { child: _, .. } => {
            // Parent write into pipe 1 - Message is: A
            let message_a = CString::new("A").unwrap();
            write(pipe1_write_fd, message_a.as_bytes_with_nul())?;
            close(pipe1_write_fd)?;

            // Parent read from pipe 2 - Message is: B
            let mut buffer = [0; 128];
            let bytes_read = read(pipe2_read_fd, &mut buffer)?;
            let received_message =
                unsafe { CString::from_vec_unchecked(buffer[..bytes_read].to_vec()) };
            println!("Parent received: {:?}", received_message);

            // Close the read file descriptor
            close(pipe2_read_fd)?;

            // Wait for the child process to complete
            wait()?;
        }
        ForkResult::Child => {
            // Child write into pipe 2 - Message is: B
            let message_b = CString::new("B").unwrap();
            write(pipe2_write_fd, message_b.as_bytes_with_nul())?;
            close(pipe2_write_fd)?;

            // Child read from pipe 1 - Message is: A
            let mut buffer = [0; 128];
            let bytes_read = read(pipe1_read_fd, &mut buffer)?;
            let received_message =
                unsafe { CString::from_vec_unchecked(buffer[..bytes_read].to_vec()) };
            println!("Child received: {:?}", received_message);

            // Close the read file descriptor
            close(pipe1_read_fd)?;

            // Exit the child process
            exit(0);
        }
    }

    Ok(())
}
