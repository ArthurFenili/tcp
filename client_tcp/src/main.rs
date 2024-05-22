use std::io::{self, Read, Write};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    println!("Connected to the server!");

    let mut buffer = [0; 1024];

    let n = stream.read(&mut buffer)?;
    println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));

    Ok(())
}
