use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    collections::HashMap,
};

#[derive(Debug)]
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

    let request = parse_request(http_request);

    println!("{}", "\n\n\n");
    println!("{:#?}", request);

    let response = handle_request(request);
    stream.write_all(response.as_bytes()).unwrap();
}

fn handle_index(_request: Request) -> String {
    let mut response = String::new();

    let contents = fs::read_to_string("index.html").unwrap();

    response.push_str("HTTP/1.1 200 OK\r\n");
    response.push_str("Content-Type: text/html\r\n");
    response.push_str("\r\n");
    response.push_str(&contents);

    response
}

fn handle_404(_request: Request) -> String {
    let mut response = String::new();

    let contents = fs::read_to_string("404.html").unwrap();

    response.push_str("HTTP/1.1 404 Not Found\r\n");
    response.push_str("Content-Type: text/html\r\n");
    response.push_str("\r\n");
    response.push_str(&contents);

    response
}

fn handle_request(request: Request) -> String {
    match (request.verb.as_str(), request.path.as_str()) {
        ("GET", "/") => handle_index(request),
        _ =>            handle_404(request),
    }
}

fn parse_request(http_request: Vec<String>) -> Request {
    let mut request = Request::new();
    let mut iterator = http_request.iter();
    let mut headline_split = iterator.next().unwrap().split(' ');

    let (verb, path) = (headline_split.next().unwrap(), 
                        headline_split.next().unwrap());

    println!("{} {}", verb, path);
    
    while let Some(line) = iterator.next() {
        println!("{}", line);

        if line.is_empty() {
            break;
        }

        let (key, value) = line.split_once(": ").unwrap();

        if key == "Cookie" {
            let cookies: Vec<_> = value.split("; ").collect();

            for cookie in cookies {
                let (key, value) = cookie.split_once("=").unwrap();
                request.cookies.insert(key.to_string(), value.to_string());
            }
        } else {
            request.headers.insert(key.to_string(), value.to_string());
        }
    }
    
    request.verb = verb.to_string();
    request.path = path.to_string();

    request
}
