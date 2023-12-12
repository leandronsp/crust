mod router;
pub(crate) mod persistence;

use std::{
    collections::HashMap,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use regex::Regex;

#[derive(Debug)]
pub struct Request {
    verb: String,
    path: String,
    headers: HashMap<String, String>,
    cookies: HashMap<String, String>,
    params: HashMap<String, String>,
    body: String,
}

impl Request {
    fn new() -> Self {
        Self {
            verb: String::new(),
            path: String::new(),
            headers: HashMap::new(),
            cookies: HashMap::new(),
            params: HashMap::new(),
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
    let mut buf_reader = BufReader::new(&mut stream);

    let http_request: Vec<_> = buf_reader
        .by_ref()
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let request = parse_request(http_request, buf_reader);

    println!("{}", "\n\n\n");
    println!("{:#?}", request);

    let response = handle_request(request);
    stream.write_all(response.as_bytes()).unwrap();
}

fn handle_request(mut request: Request) -> String {
    // Apply route constaints
    // verb: GET, POST, PUT, DELETE
    // path: /tasks/:id, replace "42" to ":id" and inject into the request.params
    // parts: tasks, (42), groups, (dashboard), report
    // match (verb, path) {

    // /tasks/42 -> /tasks/:id
    // request.params["id"] = 42

    let verb = request.verb.as_str();
    let mut path = request.path.clone();

    let constraint_pattern = Regex::new(r"^\/tasks\/(\d+)$").unwrap();

    if let Some(found) = constraint_pattern.captures(&request.path) {
        let id = found.get(1).unwrap().as_str();
        path = path.replace(id, ":id");
        request.params.insert("id".to_string(), id.to_string());
    }

    match (verb, path.as_str()) {
        ("GET", "/") => router::get::index(request),
        ("GET", "/login") => router::get::login(request),
        ("GET", "/signup") => router::get::signup(request),
        ("POST", "/login") => router::post::login(request),
        ("POST", "/signup") => router::post::signup(request),
        ("POST", "/logout") => router::post::logout(request),
        ("POST", "/tasks") => router::post::tasks(request),
        ("DELETE", "/tasks/:id") => router::delete::tasks(request),
        _ => router::get::second_lookup(request),
    }
}

fn parse_request(http_request: Vec<String>, buf_reader: BufReader<&mut TcpStream>) -> Request {
    let mut request = Request::new();
    let mut iterator = http_request.iter();
    let mut headline_split = iterator.next().unwrap().split(' ');

    let (verb, path) = (
        headline_split.next().unwrap(),
        headline_split.next().unwrap(),
    );

    request.verb = verb.to_string();
    request.path = path.to_string();

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

    // Parse Body
    if let Some(content_length) = request.headers.get("Content-Length") {
        buf_reader
            .take(content_length.parse::<u64>().unwrap())
            .read_to_string(&mut request.body)
            .unwrap();

        if !request.body.is_empty()
            && request.headers.get("Content-Type").unwrap() == "application/x-www-form-urlencoded"
        {
            let body: Vec<_> = request.body.split("&").collect();

            for pair in body {
                let (key, value) = pair.split_once("=").unwrap();
                request.params.insert(key.to_string(), value.to_string());
            }
        }

        if !request.body.is_empty()
            && request.headers.get("Content-Type").unwrap() == "application/json"
        {
            let body: HashMap<String, String> = serde_json::from_str(&request.body).unwrap();

            for (key, value) in body {
                request.params.insert(key, value);
            }
        }
    }

    request
}
