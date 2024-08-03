use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut socket = TcpStream::connect("10.100.102.24:8080").await?;

    let mut buf = vec![0u8; 1280 * 800 * 3];
    // let mut buf = Vec::new();
    let mut temp_buf = [0u8; 10000];
    let mut offset = 0;

    loop {
        let n = socket.read(&mut temp_buf).await?;
        if n == 0 {
            break; // Connection closed
        }        // Append the read data to the buffer
        // buf.extend_from_slice(&temp_buf[..n]);
        for i in 0..n {
            buf[offset + i] = temp_buf[i];
        }
        offset += n;
        // Check for JPEG end marker or buffer size (this may need refinement)
        if temp_buf[n - 2] == 0xFF && temp_buf[n - 1] == 0xD9 {
            let path = "image.jpeg";
            
            // Save the image as a JPEG file
            let mut file = File::create(&path)?;
            file.write_all(&buf)?;
            
            offset = 0;
            println!("buffer size: {}", buf.len());
            tokio::time::sleep(Duration::from_millis(100)).await;
            // buf = Vec::new();
        }
    }
    Ok(())
}