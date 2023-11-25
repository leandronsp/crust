pub mod get {
    use std::fs;

    use crate::Request;

    pub fn login(_request: Request) -> String {
        let mut response = String::new();

        let contents = fs::read_to_string("login.html").unwrap();

        response.push_str("HTTP/1.1 200 OK\r\n");
        response.push_str("Content-Type: text/html\r\n");
        response.push_str("\r\n");
        response.push_str(&contents);

        response
    }

    pub fn index(request: Request) -> String {
        let mut response = String::new();
        
        if let Some(cookie) = request.cookies.get("username") {
            let contents = fs::read_to_string("index.html").unwrap();
            let contents = contents.replace("{{username}}", cookie);

            response.push_str("HTTP/1.1 200 OK\r\n");
            response.push_str("Content-Type: text/html\r\n");
            response.push_str("\r\n");
            response.push_str(&contents);

            return response;
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

    pub fn login(request: Request) -> String {
        let mut response = String::new();

        let username = request.params.get("username").unwrap();
        let _password = request.params.get("password").unwrap();

        response.push_str("HTTP/1.1 301\r\n");
        response.push_str("Location: /\r\n");
        response.push_str(format!("Set-Cookie: username={}\r\n", username).as_str());
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
