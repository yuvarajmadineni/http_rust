use std::env::{self};
use std::io::Read;
use std::net::TcpListener;
use std::{io::Write, net::TcpStream};

use http::{parse_request, Response};
use threadpool::Threadpool;
pub mod http;
pub mod threadpool;

// building thread pool
// you need to store the number of threads you can spawn - count
// create a pool of threads , whenever a request is made , get available thread from the pool and
// execute , otherwise keep the request in a queue and execute whenever a thread is available

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    let thread_pool = Threadpool::new(4);
    while let Ok((stream, _)) = listener.accept() {
        thread_pool.execute(move || {
            handle_connection(stream);
        })
    }
}

fn handle_connection(mut stream: TcpStream) {
    let request = parse_request(&stream);
    let args: Vec<String> = env::args().collect();

    let mut dir_path = "";
    if args.len() > 2 {
        dir_path = &args[2];
    }

    let response = if request.path == "/" {
        Response::ok()
    } else if let Some(content) = request.path.strip_prefix("/echo/") {
        Response::ok()
            .set_headers(String::from("Content-Type"), String::from("text/plain"))
            .set_headers(String::from("Content-Length"), content.len().to_string())
            .set_body(content.to_string())
    } else if request.path.starts_with("/user-agent") {
        let user_agent = request.headers.get("user-agent").unwrap().trim();
        Response::ok()
            .set_headers(String::from("Content-Type"), String::from("text/plain"))
            .set_headers(String::from("Content-Length"), user_agent.len().to_string())
            .set_body(user_agent.to_string())
    } else if let Some(file_path) = request.path.strip_prefix("/files/") {
        let path = format!("{}/{}", dir_path, file_path);
        let result = std::fs::File::open(path);

        match result {
            Ok(mut file) => {
                let mut content = String::from("");
                let _ = file.read_to_string(&mut content);
                let metadata = file.metadata();
                let mut file_size = 0;

                match metadata {
                    Ok(meta) => {
                        file_size = meta.len();
                    }
                    Err(_) => {}
                }
                Response::ok()
                    .set_headers(
                        String::from("Content-Type"),
                        String::from("application/octet-stream"),
                    )
                    .set_headers(String::from("Content-Length"), file_size.to_string())
                    .set_body(content.to_string())
            }

            Err(e) => {
                eprintln!("Error while reading contents from the file {}", e);
                Response::not_found()
            }
        }
    } else {
        Response::not_found()
    };

    let mut result = Vec::new();

    let status = response.get_status_str();
    result.push(status);
    result.push("\r\n".to_string());
    for entry in response.headers {
        result.push(format!("{}: {}", entry.0, entry.1).to_string());
        result.push("\r\n".to_string());
    }

    result.push("\r\n".to_string());

    for body in response.body {
        result.push(body);
    }

    let final_response = result.join("");
    _ = stream.write_all(final_response.as_bytes());
}
