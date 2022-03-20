use argon2::{self, Config};
use dotenv::dotenv;
use jsonwebtoken::{
    decode, decode_header, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation
};
use postgres::{Client, NoTls};
use serde::de::Unexpected::Str;
use serde::Serialize;
use std::env;
use uuid::Uuid;

pub fn create_JWT<T: Serialize>(data: T) -> String {
    encode(
        &Header::default(),
        &data, // Data for JWT
        &EncodingKey::from_secret(dotenv::var("SECRET").unwrap().as_ref()),
    )
    .unwrap()
}

pub fn hash_password(password: &str) -> String {
    let config = Config::default();
    argon2::hash_encoded(
        password.as_ref(),
        dotenv::var("SALT").unwrap().as_ref(),
        &config,
    )
    .unwrap()
}

pub fn check_password<'a>(login: &'a str, password: &'a str) -> (bool, i32, Uuid) {
    let mut conn = Client::connect(&dotenv::var("DB_CONN").unwrap() as &str, NoTls).unwrap();
    let mut ability: i32 = 0;
    let mut password_hash: String = "".to_string();
    let mut id = Uuid::nil();
    for element in conn
        .query(
            "SELECT password_hash, ability, id FROM users WHERE login = $1",
            &[&login],
        )
        .unwrap()
    {
        password_hash = element.get("password_hash");
        ability = element.get("ability");
        id = element.get("id");
    }
    let res = argon2::verify_encoded(&password_hash, password.as_ref()).unwrap();
    (res, ability, id)
}

pub fn save_jwt_token(uuid: String, token: String) {
    let mut conn = Client::connect(&dotenv::var("DB_CONN").unwrap() as &str, NoTls).unwrap();
    conn.query(
        "INSERT INTO tokens(ref_id, token) VALUES($1, $2)",
        &[&uuid, &token],
    );
}

pub fn refresh_token(uuid: String, token: String) {
    let mut conn = Client::connect(&dotenv::var("DB_CONN").unwrap() as &str, NoTls).unwrap();
    conn.query(
        "UPDATE tokens SET token = $1 WHERE ref_id = $2",
        &[&token, &uuid],
    );
}
