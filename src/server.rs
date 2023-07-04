use std::io::{Read, Write};
use std::net::{TcpListener};

pub fn serve(body: String) {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("HTML served at http://{}", listener.local_addr().unwrap());

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", body.len(), body);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}