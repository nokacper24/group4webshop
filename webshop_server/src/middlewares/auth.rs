use serde::{Serialize, Deserialize};




#[derive(Debug, Serialize, Deserialize)]
pub struct Cookie {
    pub cookie_id: i32,
    pub user_id: i32,
    pub exp: i64,
    pub cookie: String,
}

pub fn check_auth(cookie: &str) -> bool {
   
}