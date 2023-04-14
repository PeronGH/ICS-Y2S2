use std::io::{Read, Write};
use std::os::unix::net::UnixStream;

pub fn main() -> std::io::Result<()> {
    // Step 1: Create a pipe
    let (mut read_end, mut write_end) = UnixStream::pair()?;

    // Step 2: Write data into the pipe
    let message = "Hello, Pipe!";
    write_end.write_all(message.as_bytes())?;

    // Step 3: Read data from the pipe
    let mut buffer = [0; 128];
    let bytes_read = read_end.read(&mut buffer)?;
    let received_message = String::from_utf8_lossy(&buffer[..bytes_read]);

    println!("Received message: {}", received_message);

    // No need for Step 4: Closing the pipe, as UnixStream will be closed automatically when they go out of scope

    Ok(())
}
