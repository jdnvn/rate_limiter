mod exp_map;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::collections::HashMap;
use crate::exp_map::ExpMap;

const UPSTREAM_HOST: &str = "127.0.0.1:8082";
const PORT: &str = "4200";

fn forward_request(mut stream: &TcpStream) -> Result<usize, std::io::Error> {
    let mut upstream_socket = TcpStream::connect(UPSTREAM_HOST).unwrap();
    let mut buffer: [u8; 4096] = [0; 4096];

    let request_bytes = stream.read(&mut buffer)?;
    println!("Read {request_bytes} bytes from client, forwarding to upstream server...");
    upstream_socket.write(&buffer)?;

    buffer.fill(0);

    let response_bytes = upstream_socket.read(&mut buffer)?;
    println!("Received {response_bytes} from upstream, responding to client...");
    stream.write(&buffer)
}

fn throttle(mut stream: &TcpStream) -> std::io::Result<()> {
    let response = "HTTP/1.1 429 Too Many Requests\r\n\
                   Content-Type: text/plain\r\n\
                   Content-Length: 17\r\n\
                   \r\n\
                   Too Many Requests";
    stream.write_all(response.as_bytes())
}

fn main() -> std::io::Result<()> {
    let mut requests = ExpMap::new();
    let host = format!("127.0.0.1:{PORT}");
    let listener = TcpListener::bind(host)?;
    println!("Listening on port {PORT}...");

    for stream in listener.incoming() {
        let stream = stream?;
        let ip = match stream.peer_addr() {
            Ok(addr) => addr.ip().to_string(),
            Err(e) => {
                eprintln!("Failed to get peer address: {}", e);
                return Err(e);
            }
        };
        println!("Received request from {ip}");
        let request_count = requests.incr(ip);
        if request_count >= 5 {
            println!("Throttling!");
            throttle(&stream);
        } else {
            forward_request(&stream);
        }
    }
    Ok(())
}
