pub mod get {
    use std::fs;

    use crate::Request;
    use crate::persistence::UsersRepository;

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
            let repository = UsersRepository::new();
            let user = repository.find_by_username(cookie);

            match user {
                Some(user) => {
                    let contents = fs::read_to_string("index.html").unwrap();
                    let contents = contents.replace("{{username}}", user.username.as_str());

                    response.push_str("HTTP/1.1 200 OK\r\n");
                    response.push_str("Content-Type: text/html\r\n");
                    response.push_str("\r\n");
                    response.push_str(&contents);

                    return response;
                },
                None => {
                    response.push_str("HTTP/1.1 301\r\n");
                    response.push_str("Location: /login\r\n");
                    response.push_str("\r\n");

                    return response;
                }
            }
        } 

        response.push_str("HTTP/1.1 301\r\n");
        response.push_str("Location: /login\r\n");
        response.push_str("\r\n");

        response
    }

    pub fn second_lookup(request: Request) -> String {
        let static_regex = regex::Regex::new(r"\/*.css$").unwrap();
        
        if let Some(_) =  static_regex.find(request.path.as_str()) {
            serve_static_file(request.path.as_str().trim_start_matches("/"))
        } else {
            not_found()
        }
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

    pub fn serve_static_file(filename: &str) -> String {
        let mut response = String::new();

        let contents = fs::read_to_string(filename).unwrap();

        response.push_str("HTTP/1.1 200 OK\r\n");
        response.push_str("Content-Type: text/css\r\n");
        response.push_str("\r\n");
        response.push_str(&contents);

        response
    }
}

pub mod post {
    use std::fs;
    use crate::Request;
    use crate::persistence::{User, UsersRepository};

    pub fn login(request: Request) -> String {
        let mut response = String::new();

        let username = request.params.get("username").unwrap();
        let password = request.params.get("password").unwrap();

        let repository = UsersRepository::new();

        let user = repository.find_by_credentials(username, password);

        match user {
            Some(user) => {
                response.push_str("HTTP/1.1 301\r\n");
                response.push_str("Location: /\r\n");
                response.push_str(format!("Set-Cookie: username={}\r\n", user.username).as_str());
                response.push_str("\r\n");

                return response;
            },
            None => {
                let contents = fs::read_to_string("401.html").unwrap();
                response.push_str("HTTP/1.1 401 Unauthorized\r\n");
                response.push_str("Content-Type: text/html\r\n");
                response.push_str("Set-Cookie: username=; Max-Age=0\r\n");
                response.push_str("\r\n");
                response.push_str(&contents);

                return response;
            }
        }
    }

    pub fn signup(request: Request) -> String {
        let mut response = String::new();

        let user = User::new(
            request.params.get("username").unwrap(),
            request.params.get("password").unwrap(),
        );

        let mut repository = UsersRepository::new();
        repository.save(user);

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
