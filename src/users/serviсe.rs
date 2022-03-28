use crate::models::user::*;
use crate::users::auth::auth;
use crate::users::auth::auth::{hash_password, send_checker_email};
use actix_web::web;
use actix_web::web::Json;
use argon2::Config;
use jsonwebtoken::{
    decode, decode_header, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::postgres::*;
use sqlx::{postgres, PgPool};
use std::time::{Duration, SystemTime};
use uuid::Uuid;

impl NewUser {
    pub fn create_user(self) -> User {
        User {
            login: self.login,
            password_hash: hash_password(&self.password),
            ability: 0,
            email: self.email,
            image: "http://assets.stickpng.com/images/585e4bcdcb11b227491c3396.png".to_string(),
            approved: false,
        }
    }
    fn hash_password(password: &str) -> String {
        let config = Config::default();
        argon2::hash_encoded(
            password.as_ref(),
            dotenv::var("SALT").unwrap().as_ref(),
            &config,
        )
        .unwrap()
    }

    pub fn for_jwt(self, ability: i32) -> UserJWT {
        UserJWT {
            login: self.login,
            ability: ability,
        }
    }
}


impl User {
    pub fn for_check_email(self) -> UserCheckEmail {
        UserCheckEmail {
            email: self.email,
            link: Uuid::new_v4().to_string()
        }
    }
}

pub async fn post_registration(
    pool: web::Data<PgPool>,
    data: NewUser,
) -> Result<String, sqlx::error::Error> {
    let user = data.clone().create_user();
    let uuid = sqlx::query(
        "INSERT INTO users2(password_hash, login, ability, email, image, approved) VALUES($1, $2, $3, $4, $5, $6) RETURNING id")
        .bind(&user.password_hash)
        .bind(&user.login)
        .bind(&user.ability)
        .bind(&user.email)
        .bind(&user.image)
        .bind(&user.approved)
        .fetch_one(&**pool).await;
    match uuid {
        Ok(res) => {
            if send_checker_email(auth::create_JWT(user.for_check_email())).await {
                Ok(auth::create_JWT(data.clone().for_jwt(0)))
            }else {
                Ok(auth::create_JWT(data.clone().for_jwt(1)))
            }
        }
        Err(res) => {
            Err(res)
        },
    }
}

pub async fn post_login(pool: web::Data<PgPool>, data: UserLogin) -> Result<String, String> {
    let match_password = auth::check_password(pool.clone(), &data.login, &data.password).await;
    let mut jwt = String::new();
    let res = if match_password.0 {
        jwt = auth::create_JWT(&data);
        auth::save_jwt_token(pool.clone(), match_password.2.to_string(), jwt.clone());
        Ok(jwt)
    } else {
        Err("Неверный логин или пароль".to_string())
    };
    res
}
