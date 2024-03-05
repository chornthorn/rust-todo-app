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

#[derive(Debug, Serialize, Deserialize)]
pub struct StdResponse {
    pub message: String,
    pub error: String,
    pub status: u16
}

impl Default for StdResponse {
    fn default() -> Self {
        StdResponse {
            message: "Success".to_string(),
            error: "".to_string(),
            status: 200
        }
    }
}