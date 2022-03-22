use serde::{Deserialize, Serialize};
use serde_json::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserRegistration {
    pub login: String,
    pub password: String,
    pub ability: i32,
    pub email: String,
}

impl UserRegistration {
    pub fn for_jwt(self) -> UserRegistrationNotPassword {
        UserRegistrationNotPassword {
            login: self.login,
            ability: self.ability,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserJWTSchema {
    pub id: String,
    pub login: String,
    pub ability: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserLogin {
    pub login: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserRegistrationNotPassword {
    pub login: String,
    pub ability: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserJWT {
    pub login: String,
    pub ability: Vec<String>,
}