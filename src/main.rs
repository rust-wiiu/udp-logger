use chrono::Local;
use std::io;
use std::net::UdpSocket;
use std::str;

const SERVER_PORT: u16 = 4405;
const BUFFER_SIZE: usize = 4096;

fn receive_udp_packets() -> io::Result<()> {
    let socket = UdpSocket::bind(format!("0.0.0.0:{}", SERVER_PORT))?;
    println!("Listening for UDP packets on port {}...", SERVER_PORT);

    let mut buffer = [0u8; BUFFER_SIZE];

    loop {
        match socket.recv_from(&mut buffer) {
            Ok((size, _addr)) => {
                if size <= 0 {
                    continue;
                }

                if let Ok(message) = str::from_utf8(&buffer[..size]) {
                    print!("{} | {}", Local::now().format("%H:%M:%S"), message);
                }
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
            }
        }
    }
}

fn main() -> io::Result<()> {
    receive_udp_packets()
}
