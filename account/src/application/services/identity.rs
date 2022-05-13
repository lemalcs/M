use crate::entities::user::User;

pub trait IdentityService {
    fn create_certificate(&self, user: User);
}