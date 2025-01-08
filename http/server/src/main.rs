use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server is running on: localhost:7878");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; // Fixed syntax: [0; 1024] instead of [0: 1024]
    
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received request:\n{}", request);
    
    let response = "HTTP/1.1 200 OK\r\n\
                    Content-Type: text/html\r\n\
                    \r\n\
                    <html>\
                    <body>\
                    <h1>Hello from Rust Server!</h1>\
                    </body>\
                    </html>";
    
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}