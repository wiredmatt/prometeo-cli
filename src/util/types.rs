use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Provider {
    pub code: String,
    pub country: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProvidersResponse {
    pub providers: Vec<Provider>,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResponse {
    pub key: String,
    pub status: String,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub document: String,
    pub email: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
struct Prefs {
    api_key: String,
    user: User,
}

impl fmt::Display for Provider {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.code)
    }
}
