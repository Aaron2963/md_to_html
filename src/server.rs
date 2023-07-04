use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Write;

pub fn serve(html: &String) {
    let url = "127.0.0.1:7878";
    let listener = TcpListener::bind(url).unwrap();
    println!("HTML served at http://{}", url);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_request(stream, html);
    }
}

fn handle_request(mut stream: TcpStream, html: &String) {
    let status_line = "HTTP/1.1 200 OK";
    let length = html.len();

    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, length, html);
    stream.write_all(response.as_bytes()).unwrap();
}
