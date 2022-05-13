use crate::entities::user::User;

pub trait UserRepository{
    fn save(&self, user: User){

    }
}