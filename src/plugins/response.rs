use actix_web::web;
use serde;
use serde_json::*;

#[derive(serde::Serialize, Debug, Clone)]
pub struct Response {
    pub error: bool,
    pub message: String,
    pub code: u16,
}

impl Response {
    pub fn new() -> Response {
        Response {
            error: false,
            message: String::new(),
            code: 0,
        }
    }

    pub fn get_error(&mut self, err: bool) {
        self.error = err;
    }

    pub fn get_message(&mut self, message: &str) {
        self.message = String::from(message);
    }

    pub fn get_code(&mut self, code: u16) {
        self.code = code;
    }
}
