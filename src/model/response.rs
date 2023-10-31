pub use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct ReturnResponse<T> {
    pub message: String,
    pub status: bool,
    pub data: Option<Vec<T>>,
}
