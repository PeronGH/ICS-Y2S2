use nix::unistd::{close, pipe, read, write};
use std::ffi::CString;

pub fn main() -> nix::Result<()> {
    // Step 1: Create a pipe
    let (read_fd, write_fd) = pipe()?;

    // Step 2: Write data into the pipe
    let message = CString::new("Hello, Pipe!").unwrap();
    write(write_fd, message.as_bytes_with_nul())?;

    // Close the write file descriptor
    close(write_fd)?;

    // Step 3: Read data from the pipe
    let mut buffer = [0; 128];
    let bytes_read = read(read_fd, &mut buffer)?;
    let received_message = unsafe { CString::from_vec_unchecked(buffer[..bytes_read].to_vec()) };

    println!("Received message: {:?}", received_message);

    // Step 4: Close the pipe
    close(read_fd)?;

    Ok(())
}
