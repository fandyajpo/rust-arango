#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: usize,
    iat: usize,
}

async fn create_token() -> Result<String, StatusCode> {}

async fn verify_token() -> Result<bool, StatusCode> {}
