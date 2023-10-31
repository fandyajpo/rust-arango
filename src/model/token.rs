use serde::Deserialize;
use serde::Serialize;
#[derive(Serialize, Debug, Deserialize)]
pub struct Token {
    pub access_token: String,
    pub refresh_token: String,
}
