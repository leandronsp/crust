use std::sync::Mutex;

pub static DATABASE: Mutex<Vec<User>> = Mutex::new(Vec::new());

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

    pub fn save(&self) {
        let mut store = DATABASE.lock().unwrap();

        store.push(User {
            username: self.username.clone(),
            password: self.password.clone(),
        });
    }

    pub fn find_by_username(username: &str) -> Option<User> {
        let store = DATABASE.lock().unwrap();

        // Full-table scan
        store
            .iter()
            .find(|user| user.username == username)
            .map(|user| User {
                username: user.username.clone(),
                password: user.password.clone(),
            })
    }

    pub fn find_by_credentials(username: &str, password: &str) -> Option<User> {
        let store = DATABASE.lock().unwrap();

        // Full-table scan
        store
            .iter()
            .find(|user| user.username == username && user.password == password)
            .map(|user| User {
                username: user.username.clone(),
                password: user.password.clone(),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_save() {
        let user = User::new("leandro senior", "password");
        user.save();

        let store = DATABASE.lock().unwrap();

        store.iter().for_each(|user| {
            assert_eq!(user.username, "leandro senior");
            assert_eq!(user.password, "password");
        });
    }

    #[test]
    fn user_find_by_username() {
        let user = User::new("leandro senior", "password");
        user.save();

        let user = User::find_by_username("leandro senior").unwrap();

        assert_eq!(user.username, "leandro senior");
        assert_eq!(user.password, "password");
    }

    #[test]
    fn user_find_by_credentials() {
        let user = User::new("leandro senior", "password");
        user.save();

        let user = User::find_by_credentials("leandro senior", "password").unwrap();

        assert_eq!(user.username, "leandro senior");
        assert_eq!(user.password, "password");
    }
}
