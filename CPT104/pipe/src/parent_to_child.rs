use nix::sys::wait::wait;
use nix::unistd::{fork, pipe, read, write, ForkResult};
use std::ffi::CString;
use std::process::exit;

pub fn main() -> nix::Result<()> {
    // Step 1: Create a pipe
    let (read_fd, write_fd) = pipe()?;

    // Step 2: Create a child process
    match unsafe { fork() }? {
        ForkResult::Parent { child: _, .. } => {
            // Step 3: Write the data into the pipe using the parent process
            let message = CString::new("Hello, Child!").unwrap();
            write(write_fd, message.as_bytes_with_nul())?;

            // Close the write file descriptor in the parent process
            nix::unistd::close(write_fd)?;

            // Wait for the child process to complete
            wait()?;
        }
        ForkResult::Child => {
            // Step 4: Read the data from the pipe using the child process
            // Close the write file descriptor in the child process
            nix::unistd::close(write_fd)?;

            let mut buffer = [0; 128];
            let bytes_read = read(read_fd, &mut buffer)?;
            let received_message =
                unsafe { CString::from_vec_unchecked(buffer[..bytes_read].to_vec()) };
            println!("Child received: {:?}", received_message);

            // Close the read file descriptor in the child process
            nix::unistd::close(read_fd)?;

            // Exit the child process
            exit(0);
        }
    }

    Ok(())
}
