use serde::{Serialize, Deserialize};
use sqlx::{query_as,{Postgres, Pool}};


pub struct User {
    user_id: i32,
    email: String,
    pass_hash: String,
    company_id: i32,
    role: Role
}
