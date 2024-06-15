use std::net::TcpStream;
use std::io::{self, Write, Read};
use std::thread;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    println!("Successfully connected to server in port 7878");

    // thread para receber as mensagens do servidor a qualquer momento que forem enviadas
    let mut stream_clone = stream.try_clone().expect("Failed to clone stream");
    thread::spawn(move || {
        let mut buffer = [0; 512];
        loop {
            match stream_clone.read(&mut buffer) {
                Ok(size) => {
                    if size > 0 {
                        println!("{}", String::from_utf8_lossy(&buffer[..size]));
                    }
                }
                Err(_) => {
                    println!("Connection closed by server");
                    break;
                }
            }
        }
    });

    // loop sem thread para esperar o cliente escrever uma mensagem
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let msg = input.trim().as_bytes();
        stream.write_all(msg)?;

        if input.trim() == "END/" {
            println!("Disconnecting...");
            break;
        }
    }
    Ok(())
}
