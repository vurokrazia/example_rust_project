use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use  std::fs;

fn main() {

	let listener = TcpListener::bind("0.0.0.0:7373").unwrap();
    	for stream in listener.incoming() {
        	let stream = stream.unwrap();
        	handle_connection(stream);
    	}
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let get = b"GET / HTTP/1.1\r\n";
    stream.read(&mut buffer).unwrap();
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    if buffer.starts_with(get) {
        let contents = fs::read_to_string("src/index.html").unwrap();
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
        stream.write(response.as_bytes()).unwrap(); //Nos ayuda a leer la cadena bytes que estamos recibiendo.
        stream.flush().unwrap(); //Esperará e impedirá que el programa continúe hasta que se escriban todos los bytes en la conexión.
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    	let contents = fs::read_to_string("src/404.html").unwrap();

    	let response = format!("{}{}", status_line, contents);

    	stream.write(response.as_bytes()).unwrap();
    	stream.flush().unwrap();
    }
}