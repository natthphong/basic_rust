use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct CustomerInfo {
    id: i64,
    age: i32,
    email: String,
    password: String,
    username: String,
    is_delete: String,
}

impl CustomerInfo {
    pub fn new(id: i64, age: i32, email: String, password: String, username: String, is_delete: String) -> Self {
        Self {
            id,
            age,
            email,
            password,
            username,
            is_delete,
        }
    }

    pub fn get_id(&self)->&i64{
        return &self.id;
    }

}
