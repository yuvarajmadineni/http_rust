use std::io::{BufRead, BufReader, Write};
#[allow(unused_imports)]
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                let buf_reader = BufReader::new(&mut _stream);
                let request_line = buf_reader.lines().next().unwrap().unwrap();
                let response = match request_line.as_str() {
                    "GET / HTTP/1.1" => "HTTP/1.1 200 OK\r\n\r\n",
                    _ => "HTTP/1.1 404 Not Found\r\n\r\n",
                };
                _stream.write_all(response.as_bytes()).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
