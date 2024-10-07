use std::io::Write;
use std::net::TcpListener;

use http::{parse_request, Response};
pub mod http;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    while let Ok((mut stream, _)) = listener.accept() {
        let request = parse_request(&stream);

        let response = if request.path == "/" {
            Response::ok()
        } else if let Some(content) = request.path.strip_prefix("/echo/") {
            Response::ok()
                .set_headers(String::from("Content-Type"), String::from("text/plain"))
                .set_headers("Content-Length".to_string(), content.len().to_string())
                .set_body(content.to_string())
        } else if request.path.starts_with("/user-agent") {
            let user_agent = request.headers.get("user-agent").unwrap().trim();
            Response::ok()
                .set_headers(String::from("Content-Type"), String::from("text/plain"))
                .set_headers("Content-Length".to_string(), user_agent.len().to_string())
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
}
