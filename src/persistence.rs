use std::fs;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub password: String,
}

impl User {
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}

pub struct UsersRepository {
    pub store: Vec<User>,
}

impl UsersRepository {
    pub fn new() -> Self {
        let mut repository = Self { store: Vec::new() };
        repository.load_from_disk();

        repository
    }

    pub fn save(&mut self, user: User) {
        self.store.push(user);
    }

    pub fn find_by_username(&self, username: &str) -> Option<User> {
        // Full-table scan
        self.store
            .iter()
            .find(|user| user.username == username)
            .map(|user| User {
                username: user.username.clone(),
                password: user.password.clone(),
            })
    }

    pub fn find_by_credentials(&self, username: &str, password: &str) -> Option<User> {
        // Full-table scan
        self.store
            .iter()
            .find(|user| user.username == username && user.password == password)
            .map(|user| User {
                username: user.username.clone(),
                password: user.password.clone(),
            })
    }

    pub fn load_from_disk(&mut self) {
        let data = fs::read_to_string("database.json").unwrap();
        let users: Vec<User> = serde_json::from_str(&data).unwrap();

        self.store.extend(users);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_find_by_username() {
        let mut repository = UsersRepository::new();

        let user = User::new("leandro", "password");

        repository.save(user);

        let user = repository.find_by_username("leandro").unwrap();

        assert_eq!(user.username, "leandro");
        assert_eq!(user.password, "password");
    }

    #[test]
    fn user_find_by_credentials() {
        let mut repository = UsersRepository::new();

        let user = User { 
            username: "leandro".to_string(), 
            password: "password".to_string() 
        };

        repository.save(user);

        let user = repository.find_by_credentials("leandro", "password").unwrap();

        assert_eq!(user.username, "leandro");
        assert_eq!(user.password, "password");
    }
}
