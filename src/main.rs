use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    collections::HashMap,
};

struct Request {
    verb: String,
    path: String,
    headers: HashMap<String, String>,
    cookies: HashMap<String, String>,
    body: String,
}

impl Request {
    fn new() -> Self {
        Self {
            verb: String::new(),
            path: String::new(),
            headers: HashMap::new(),
            cookies: HashMap::new(),
            body: String::new(),
        }
    }
}

fn main() {
    let listener = TcpListener::bind("localhost:3000").unwrap();

    println!("Server listening on port 3000");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("{}", "\n\n\n");
    http_request.iter().for_each(|line| println!("{}", line));

    let request = parse_request(http_request);
    assert_eq!(request.verb, "GET");
    assert_eq!(request.path, "/");

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("index.html").unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\nContent-Type: text/html\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn parse_request(http_request: Vec<String>) -> Request {
    let mut request = Request::new();
    let mut iterator = http_request.iter();

    let mut headline_split = iterator.next().unwrap().split(' ');
    let (verb, path) = (headline_split.next().unwrap(), headline_split.next().unwrap());
    
    request.verb = String::from(verb);
    request.path = String::from(path);

    request
}
