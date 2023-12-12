pub mod get {
    use std::fs;

    use crate::Request;
    use crate::persistence::{UsersRepository, TasksRepository};

    pub fn login(request: Request) -> String {
        let mut response = String::new();

        response.push_str("HTTP/1.1 200 OK\r\n");
        response.push_str("Content-Type: text/html\r\n");

        let mut contents = fs::read_to_string("src/views/html/login.html").unwrap();

        if let Some(message) = request.cookies.get("flash") {
            contents = contents.replace("<y-flash/>", format!("<p class='flash'>{message}</p>").as_str());
            response.push_str("Set-Cookie: flash=; Max-Age=0\r\n");
        }

        response.push_str("\r\n");
        response.push_str(&contents);

        response
    }

    pub fn signup(request: Request) -> String {
        let mut response = String::new();

        let mut contents = fs::read_to_string("src/views/html/signup.html").unwrap();

        response.push_str("HTTP/1.1 200 OK\r\n");
        response.push_str("Content-Type: text/html\r\n");

        if let Some(message) = request.cookies.get("flash") {
            contents = contents.replace("<y-flash/>", format!("<p class='flash'>{message}</p>").as_str());
            response.push_str("Set-Cookie: flash=; Max-Age=0\r\n");
        }

        response.push_str("\r\n");
        response.push_str(&contents);

        response
    }

    pub fn index(request: Request) -> String {
        let mut response = String::new();
        
        if let Some(cookie) = request.cookies.get("username") {
            let repository = UsersRepository::new();
            let tasks_repository = TasksRepository::new();

            let user = repository.find_by_username(cookie);

            match user {
                Some(user) => {
                    let contents = fs::read_to_string("src/views/html/index.html").unwrap();

                    let tasks = tasks_repository.all();
                    let tasks_partial = 
                        tasks
                        .iter()
                        .filter(|task| task.user_id == user.id)
                        .map(|task| {
                            format!("<div class='item'><p>{}</p><span class='material-icons' onclick='deleteTask(this)' data-task-id='{}'>delete</span></div>", task.name, task.id)
                        });

                    let contents = 
                        contents
                        .replace("{{username}}", user.username.as_str())
                        .replace("<y-tasks/>", tasks_partial.collect::<String>().as_str());

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
        let static_regex = regex::Regex::new(r"\/public/.*?/*.(css|js)$").unwrap();
        
        if let Some(_) =  static_regex.find(request.path.as_str()) {
            serve_static_file(request.path.as_str().trim_start_matches("/"))
        } else {
            not_found()
        }
    }

    pub fn not_found() -> String {
        let mut response = String::new();

        let contents = fs::read_to_string("src/views/html/404.html").unwrap();

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
    use crate::persistence::{User, UsersRepository, TasksRepository, Task};

    pub fn login(request: Request) -> String {
        let mut response = String::new();
        let repository = UsersRepository::new();

        let username = request.params.get("username").unwrap();
        let password = request.params.get("password").unwrap();

        if let Some(user) = repository.find_by_username(username) {
            if let Ok(true) = bcrypt::verify(password, user.password.as_str()) {
                response.push_str("HTTP/1.1 301\r\n");
                response.push_str("Location: /\r\n");
                response.push_str(format!("Set-Cookie: username={}\r\n", user.username).as_str());
                response.push_str("\r\n");

                return response;
            }
        }

        response.push_str("HTTP/1.1 301\r\n");
        response.push_str("Set-Cookie: flash=Invalid Credentials\r\n");
        response.push_str("Location: /login\r\n");
        response.push_str("\r\n");

        response
    }

    pub fn signup(request: Request) -> String {
        let mut response = String::new();

        let username = request.params.get("username").unwrap();
        let password = request.params.get("password").unwrap();
        let password_confirmation = request.params.get("password_confirmation").unwrap();

        if password != password_confirmation {
            response.push_str("HTTP/1.1 301\r\n");
            response.push_str("Set-Cookie: flash=Passwords do not match\r\n");
            response.push_str("Location: /signup\r\n");
            response.push_str("\r\n");

            return response;
        }

        let mut repository = UsersRepository::new();

        let last_id = repository
            .all()
            .iter()
            .fold(0, |acc, user| {
                if user.id > acc { user.id } else { acc }
            });

        let encrypted_password = bcrypt::hash(password, 4).unwrap();
        let user = User::new(last_id + 1, username, encrypted_password.as_str());

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

    pub fn tasks(request: Request) -> String {
        let mut response = String::new();
        let users_repository = UsersRepository::new();

        let username = request.cookies.get("username").unwrap();
        let current_user = users_repository.find_by_username(username).unwrap();

        let mut repository = TasksRepository::new();

        let last_id = repository
            .all()
            .iter()
            .fold(0, |acc, task| {
                if task.id > acc { task.id } else { acc }
            });

        let task = Task::new(
            last_id + 1,
            current_user.id,
            request.params.get("name").unwrap(),
        );

        repository.save(task);

        let tasks = repository.all();

        let body = tasks
            .iter()
            .filter(|task| task.user_id == current_user.id)
            .map(|task| {
                format!("<div class='item'><p>{}</p><span class='material-icons' onclick='deleteTask(this)' data-task-id='{}'>delete</span></div>", task.name, task.id)
            });

        response.push_str("HTTP/1.1 200\r\n");
        response.push_str("\r\n");
        response.push_str("\r\n");
        response.push_str(body.collect::<String>().as_str());

        response
    }
}

pub mod delete {
    use crate::Request;
    use crate::persistence::TasksRepository;

    pub fn tasks(request: Request) -> String {
        let mut response = String::new();

        let mut repository = TasksRepository::new();
        let id = request.params.get("id").unwrap().parse::<u32>().unwrap();
        repository.delete(id);

        response.push_str("HTTP/1.1 200 OK\r\n");
        response.push_str("\r\n");

        response
    }
}
