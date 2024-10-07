use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    net::TcpStream,
};

#[derive(Default, Clone, Debug)]
pub enum Mode {
    #[default]
    GET,
    POST,
}

#[derive(Default, Clone, Debug)]
pub struct Request {
    pub path: String,
    pub headers: HashMap<String, String>,
    pub mode: Mode,
}

#[derive(Default, Clone, Copy)]
pub enum Status {
    #[default]
    OK,
    NOTFOUND,
}

#[derive(Default)]
pub struct Response {
    pub status: Status,
    pub headers: HashMap<String, String>,
    pub body: Vec<String>,
}

impl Response {
    pub fn ok() -> Self {
        Self {
            status: Status::OK,
            ..Default::default()
        }
    }

    pub fn not_found() -> Self {
        Self {
            status: Status::NOTFOUND,
            ..Default::default()
        }
    }

    pub fn set_headers(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }

    pub fn get_status_str(&self) -> String {
        match self.status {
            Status::OK => format!("HTTP/1.1 200 OK"),
            Status::NOTFOUND => format!("HTTP/1.1 404 Not Found"),
        }
    }

    pub fn set_body(mut self, body: String) -> Self {
        self.body.push(body);
        self
    }
}

pub fn parse_request(stream: &TcpStream) -> Request {
    let mut buf_reader = BufReader::new(stream);

    let mut line = String::new();

    let _read_first_line = buf_reader.read_line(&mut line);
    let mut path_iter = line.split(" ");
    let mut mode: Mode = Mode::GET;
    let mut path = "/";

    if let Some(req_mode) = path_iter.next() {
        mode = match req_mode {
            "GET" => Mode::GET,
            "POST" => Mode::POST,
            _ => Mode::GET,
        }
    }

    if let Some(req_path) = path_iter.next() {
        path = req_path
    }

    let all_headers = buf_reader
        .lines()
        .map(|res| res.unwrap())
        .take_while(|line| !line.is_empty())
        .collect::<Vec<String>>();

    let mut headers = HashMap::new();

    for entry in all_headers {
        let mut entries = entry.split(":");
        let key = entries.next().unwrap();
        let value = entries.next().unwrap();
        headers.insert(key.to_lowercase(), value.to_string());
    }

    Request {
        mode,
        path: path.to_string(),
        headers,
    }
}
