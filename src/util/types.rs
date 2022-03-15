use core::fmt;

use serde::{Serialize, Deserialize};


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

impl fmt::Display for Provider {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.code)
    }
}