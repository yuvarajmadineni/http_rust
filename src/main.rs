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
                let req: Vec<_> = request_line.split(" ").collect();
                let req_url = req[1];
                let response;
                let mut dynamic_path = "";
                let mut dynamic_path_len = 0;

                let path = req_url.split("/").collect::<Vec<_>>();

                if path.len() >= 3 {
                    dynamic_path = path[2];
                    dynamic_path_len = dynamic_path.len();
                }
                if req_url.starts_with("/etc") {
                    response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", dynamic_path_len, dynamic_path)
                } else {
                    response = match request_line.as_str() {
                        "GET / HTTP/1.1" => "HTTP/1.1 200 OK\r\n\r\n".to_string(),
                        _ => "HTTP/1.1 404 Not Found\r\n\r\n".to_string(),
                    };
                }
                _stream.write_all(response.as_bytes()).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
