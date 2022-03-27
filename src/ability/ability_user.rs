// use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
// use crate::users::schema::UserJWTSchema;
// use std::env;
// use dotenv::dotenv;
//
// pub fn check_ability(jwt_token: String) {
//     let mut token_message = decode::<UserJWTSchema>(
//         &jwt_token,
//         &DecodingKey::from_secret(dotenv::var("SECRET").unwrap().as_ref()),
//         &Validation::new(Algorithm::HS256),
//     ).unwrap();
//     println!("{:?}", token_message.claims);
// }
