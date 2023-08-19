use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserLogin {
    pub username: String,
    pub password: String
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct UserRegister {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub username: String,
    pub password: String
}