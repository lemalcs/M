use crate::entities::user::User;
use crate::application::repos::auth_repo::AuthenticationRepo;

pub struct AuthenticateUserCommand{
    repo: Box<dyn AuthenticationRepo>,
}

impl  AuthenticateUserCommand {
    pub fn login(user: User ){ 

    }
}