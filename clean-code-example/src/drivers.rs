use std::collections::HashMap;

use crate::{entities::User, use_cases::UserRepository};

pub struct InMemoryUserRepository {
    users: HashMap<u32, User>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        InMemoryUserRepository {
            users: HashMap::new(),
        }
    }
}

impl UserRepository for InMemoryUserRepository {
    fn get_user(&self, id: u32) -> Option<User> {
        self.users.get(&id).cloned()
    }

    fn add_user(&self, user: User) -> Result<(), String> {
        self.users.insert(user.id, user);
        Ok(())
    }
}
