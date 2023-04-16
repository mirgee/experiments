use crate::entities::User;

pub trait UserRepository {
    fn get_user(&self, id: u32) -> Option<User>;
    fn add_user(&self, user: User) -> Result<(), String>;
}

#[derive(Clone)]
pub struct UserUseCase<'a> {
    user_repository: &'a dyn UserRepository,
}

impl<'a> UserUseCase<'a> {
    pub fn new(user_repository: &'a dyn UserRepository) -> Self {
        UserUseCase { user_repository }
    }

    pub fn get_user(&self, id: u32) -> Option<User> {
        self.user_repository.get_user(id)
    }

    pub fn add_user(&self, name: String, email: String) -> Result<User, String> {
        let user = User { id: 0, name, email };
        self.user_repository.add_user(user.clone())?;
        Ok(user)
    }
}
