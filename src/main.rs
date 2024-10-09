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
        thread_pool.execute(|| {
            handle_connection(stream);
        })
    }
}

fn handle_connection(mut stream: TcpStream) {
    let request = parse_request(&stream);

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
