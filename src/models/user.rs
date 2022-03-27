use serde::{Deserialize, Serialize};
use serde_json::*;
use uuid::Uuid;

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone)]
pub struct User {
	pub login: String,
	#[serde(skip_serializing)]
	pub password_hash: String,
	pub ability: i32,
	pub email: String,
	pub image: String,
	pub approved: bool
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NewUser {
	pub login: String,
	pub email: String,
	pub password: String,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone)]
pub struct UpdateUser {
	pub image: String
}


#[derive(Serialize, Deserialize, Clone)]
pub struct UserJWT {
	pub login: String,
	pub ability: i32,
}