pub mod constant;
pub mod response;

use actix_web::{HttpRequest, HttpResponse, Responder};
use regex::Regex;
use serde::{Deserialize, Serialize};

pub struct Regexes {}
impl Regexes {

    pub fn new(pattern: &str) -> Regex {
        Regex::new(pattern).unwrap()
    }

    pub fn username_regex() -> Regex {
        Regexes::new(r"^[a-zA-Z0-9_]+$")
    }

    pub fn email_regex() -> Regex {
        Regexes::new(r"^[a-zA-Z0-9_]+@[a-zA-Z0-9_]+\.[a-zA-Z0-9_]+$")
    }

    pub fn id_regex() -> Regex {
        Regexes::new(r"^[0-9]+$")
    }
}