use crate::entities::user::User;
pub trait AuthenticationRepo{
    fn save_user(&self, user: User);
}