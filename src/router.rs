use std::sync::Mutex;

pub struct User {
    username: String,
    password: String,
}

static DATABASE: Mutex<Vec<User>> = Mutex::new(Vec::new());

pub mod get {
    use std::fs;

    use crate::Request;

    use super::DATABASE;

    pub fn login(_request: Request) -> String {
        let mut response = String::new();

        let contents = fs::read_to_string("login.html").unwrap();

        response.push_str("HTTP/1.1 200 OK\r\n");
        response.push_str("Content-Type: text/html\r\n");
        response.push_str("\r\n");
        response.push_str(&contents);

        response
    }

    pub fn signup(_request: Request) -> String {
        let mut response = String::new();

        let contents = fs::read_to_string("signup.html").unwrap();

        response.push_str("HTTP/1.1 200 OK\r\n");
        response.push_str("Content-Type: text/html\r\n");
        response.push_str("\r\n");
        response.push_str(&contents);

        response
    }

    pub fn index(request: Request) -> String {
        let mut response = String::new();
        
        if let Some(cookie) = request.cookies.get("username") {
            let store = DATABASE.lock().unwrap();

            if let Some(user) = store.iter().find(|user| user.username == *cookie) {
                let contents = fs::read_to_string("index.html").unwrap();
                let contents = contents.replace("{{username}}", user.username.as_str());

                response.push_str("HTTP/1.1 200 OK\r\n");
                response.push_str("Content-Type: text/html\r\n");
                response.push_str("\r\n");
                response.push_str(&contents);

                return response;
            } 
        } 

        response.push_str("HTTP/1.1 301\r\n");
        response.push_str("Location: /login\r\n");
        response.push_str("\r\n");

        response
    }

    pub fn not_found() -> String {
        let mut response = String::new();

        let contents = fs::read_to_string("404.html").unwrap();

        response.push_str("HTTP/1.1 404 Not Found\r\n");
        response.push_str("Content-Type: text/html\r\n");
        response.push_str("\r\n");
        response.push_str(&contents);

        response
    }
}

pub mod post {
    use crate::Request;

    use super::{DATABASE, User};

    pub fn login(request: Request) -> String {
        let mut response = String::new();

        let username = request.params.get("username").unwrap();
        let password = request.params.get("password").unwrap();

        let store = DATABASE.lock().unwrap();
        let user = store.iter().find(|user| user.username == *username && user.password == *password);

        match user {
            Some(user) => {
                response.push_str("HTTP/1.1 301\r\n");
                response.push_str("Location: /\r\n");
                response.push_str(format!("Set-Cookie: username={}\r\n", user.username).as_str());
                response.push_str("\r\n");
            },
            None => {
                response.push_str("HTTP/1.1 301\r\n");
                response.push_str("Location: /login\r\n");
                response.push_str("\r\n");
            }
        }

        response
    }

    pub fn signup(request: Request) -> String {
        let mut response = String::new();

        let username = request.params.get("username").unwrap();
        let password = request.params.get("password").unwrap();

        let mut store = DATABASE.lock().unwrap();

        store.push(User {
            username: username.to_string(),
            password: password.to_string(),
        });

        response.push_str("HTTP/1.1 301\r\n");
        response.push_str("Location: /login\r\n");
        response.push_str("\r\n");

        response
    }

    pub fn logout(_request: Request) -> String {
        let mut response = String::new();

        response.push_str("HTTP/1.1 301\r\n");
        response.push_str("Location: /\r\n");
        response.push_str("Set-Cookie: username=; Max-Age=0\r\n");
        response.push_str("\r\n");

        response
    }
}
