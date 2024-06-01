use std::net::{TcpListener, TcpStream};
use std::io::{self, Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;

fn handle_client(stream: TcpStream, client_number: i32, clients: Arc<Mutex<Vec<TcpStream>>>) {
    let mut buffer = [0; 512];
    // loop para receber as mensagens dos clientes
    loop {
        match stream.try_clone().unwrap().read(&mut buffer) {
            Ok(size) => {
                if size > 0 {
                    if let Ok(received) = std::str::from_utf8(&buffer[0..size]) {
                        if received.starts_with("END/") {
                            let message = format!("CLIENT {} disconnected.", client_number);
                            println!("{}", message);
                            let mut clients_guard = clients.lock().unwrap();
                            if let Some(pos) = clients_guard.iter().position(|x| x.peer_addr().unwrap() == stream.peer_addr().unwrap()) {
                                clients_guard.remove(pos);
                            }
                            for mut client in clients_guard.iter() {
                                if client.peer_addr().unwrap() != stream.peer_addr().unwrap() {
                                    client.write_all(message.as_bytes()).unwrap();
                                }
                            }
                            break;
                        }
                        else if received.starts_with("CHAT/ ") {
                            let msg = &received[6..];
                            println!("CLIENT {}: {}", client_number, msg);
                            let message = format!("CLIENT {}: {}", client_number, msg);

                            let clients_guard = clients.lock().unwrap();
                            for mut client in clients_guard.iter() {
                                if client.peer_addr().unwrap() != stream.peer_addr().unwrap() {
                                    client.write_all(message.as_bytes()).unwrap();
                                }
                            }
                        }
                    } else {
                        println!("CLIENT {}: Received non-UTF8 data", client_number);
                    }
                }
            }
            Err(_) => {
                println!("An error occurred, terminating connection with CLIENT {}", client_number);
                let mut clients_guard = clients.lock().unwrap();
                if let Some(pos) = clients_guard.iter().position(|x| x.peer_addr().unwrap() == stream.peer_addr().unwrap()) {
                    clients_guard.remove(pos);
                }
                break;
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Server listening on port 7878");
    let clients = Arc::new(Mutex::new(Vec::new()));
    let mut client_number: i32 = 1;

    // thread para receber as conexões dos clientes a qualquer momento
    let clients_for_server = Arc::clone(&clients);
    thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let client_number_clone = client_number;
                    let clients = Arc::clone(&clients_for_server);
                    if let Ok(addr) = stream.peer_addr() {
                        println!("Client connected: {}", addr);
                    } else {
                        println!("Could not get client address");
                    }
                    clients.lock().unwrap().push(stream.try_clone().unwrap());
                    thread::spawn(move || {
                        handle_client(stream, client_number_clone, clients);
                    });
                    client_number += 1;
                }
                Err(e) => {
                    eprintln!("Connection failed: {}", e);
                }
            }
        }
    });

    // loop sem thread para esperar o servidor escrever uma mensagem
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let message = format!("SERVER: {}", input.trim());
        let clients_guard = clients.lock().unwrap();
        for mut client in clients_guard.iter() {
            client.write_all(message.as_bytes()).unwrap();
        }
    }
    Ok(())
}
