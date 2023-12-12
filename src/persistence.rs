use std::fs;
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub name: String,
}

impl User {
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}

impl Task {
    pub fn new(id: u32, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
        }
    }
}

pub struct UsersRepository {
    pub store: Vec<User>,
}

pub struct TasksRepository {
    pub store: Vec<Task>,
}

impl UsersRepository {
    pub fn new() -> Self {
        let mut repository = Self { store: Vec::new() };
        repository.load_from_disk();

        repository
    }

    pub fn save(&mut self, user: User) {
        self.store.push(user);
        self.persist_to_disk();
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
        let data = fs::read_to_string("users.json").unwrap();
        let users: Vec<User> = serde_json::from_str(&data).unwrap();

        self.store.extend(users);
    }

    pub fn persist_to_disk(&self) {
        let data = serde_json::to_string(&self.store).unwrap();
        fs::write("users.json", data).unwrap();
    }
}

impl TasksRepository {
    pub fn new() -> Self {
        let mut repository = Self { store: Vec::new() };
        repository.load_from_disk();

        repository
    }

    pub fn load_from_disk(&mut self) {
        let data = fs::read_to_string("tasks.json").unwrap();
        let tasks: Vec<Task> = serde_json::from_str(&data).unwrap();

        self.store.extend(tasks);
    }

    pub fn persist_to_disk(&self) {
        let data = serde_json::to_string(&self.store).unwrap();
        fs::write("tasks.json", data).unwrap();
    }

    pub fn delete(&mut self, id: u32) {
        self.store.retain(|task| task.id != id); // delete on memory
        self.persist_to_disk();
    }

    pub fn all(&self) -> Vec<Task> {
        self.store.clone()
    }

    pub fn save(&mut self, task: Task) {
        self.store.push(task);
        self.persist_to_disk();
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

    #[test]
    fn user_find_by_credentials_default() {
        let repository = UsersRepository::new();

        let user = repository.find_by_credentials("leandro", "password").unwrap();

        assert_eq!(user.username, "leandro");
        assert_eq!(user.password, "password");
    }
}
