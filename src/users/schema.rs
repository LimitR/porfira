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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Some_json {
    pub collection: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IdUserString {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IdUserI32 {
    pub id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserPub {
    pub id: i16,
    pub login: String,
    pub first_name: String,
    pub last_name: String,
    pub link_from_db_id: i16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SomeData {
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostText {
    pub login: String,
    pub text: String,
    pub img: Vec<String>,
}
