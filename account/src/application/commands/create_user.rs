use std::{fmt::Result, error::Error};

use crate::entities::user::User;
use crate::application::repos::user_repo::UserRepository;
use crate::application::services::identity::IdentityService;
pub struct CreateUserCommand{
    repo: Box<dyn UserRepository>,
    identity: Box<dyn IdentityService>,
}

impl CreateUserCommand{
    pub fn new(&self, repository: Box<dyn UserRepository>, identityServ: Box<dyn IdentityService>)->Self{
        Self{
            repo: repository, 
            identity: identityServ,
        }
    }
    pub fn handle(&self, us: User) -> bool{
        true
    }
}