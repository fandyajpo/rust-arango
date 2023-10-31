use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub struct Users {
    pub username: Option<String>,
    pub password: Option<String>,
    pub _key: Option<String>,
}
