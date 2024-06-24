use std::net::{TcpListener, TcpStream};
use std::io::{self, Read, Write, Seek, SeekFrom};
use std::sync::{Arc, Mutex};
use std::thread;
use std::fs::File;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use bincode;

#[derive(Debug, Serialize, Deserialize)]
struct Packet {    
    sequence_number: u32,
    data: Vec<u8>,
    sha: String, 
}

impl Packet {
    fn new(sequence_number: u32, data: Vec<u8>, sha: String) -> Self {
        Packet { sequence_number, data, sha }
    }
}

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    println!("Successfully connected to server in port 7878");

    // thread para receber as mensagens do servidor a qualquer momento que forem enviadas
    let mut stream_clone = stream.try_clone().expect("Failed to clone stream");
    thread::spawn(move || {
        let mut buffer = [0; 4096];
        let mut number = 0;
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

        if input.trim().starts_with("CHAT/ ") {
            let msg = input.trim().as_bytes();
            stream.write_all(msg)?;
        } 
        else if input.trim().starts_with("FILE/ ") {
            let request = input.trim().as_bytes();
            stream.write_all(request)?;

            loop {
                let mut buffer = [0; 10000];
                match stream.read(&mut buffer) {
                    Ok(size) => {
                        bincode::deserialize::<String>(&buffer[..size])
                        if size > 0 {

                        }
                    }
                    Err(_) => {
                        println!("Error receiving file data");
                        break;
                    }
                }
            }

        }
        else if input.trim() == "END/" {
            println!("Disconnecting...");
            break;
        }
    }
    Ok(())
}
