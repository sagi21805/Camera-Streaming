mod camera;
use camera::{CamSettings, Camera};
use tokio::net::{TcpStream, TcpListener};
use tokio::io::AsyncWriteExt;
use opencv::prelude::*;
use opencv::videoio::{VideoCapture, CAP_ANY};
use opencv::imgcodecs::imencode;
use opencv::core::{Vector, Mat};
use std::error::Error;
use std::time::Duration;
use std::thread::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Open the default camera

    let listener = TcpListener::bind("10.100.102.24:8080").await?;
    let settings = CamSettings::default();
    let mut cam = Camera::new(0, &settings);

    println!("waiting for connection");
    let (mut socket, _) = listener.accept().await?;
    println!("connection recived");

    loop {
    // Capture a frame
        cam.update_frame();
        // Encode the frame as JPEG
        let mut buf = Vector::<u8>::new();
        println!("size: {}", buf.len());
        imencode(".jpg", &cam.frame, &mut buf, &Vector::new())?;

        // Connect to the server
        // Send the image
        socket.write_all(buf.as_slice()).await?;
        println!("Image sent successfully");
        sleep(Duration::from_millis(50));
    }
    Ok(())
}

