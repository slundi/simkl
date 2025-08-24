use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct DeviceCodeRequest {
    pub client_id: String,
    pub redirect_uri: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeviceCodeResponse {
    pub device_code: String,
    pub user_code: String,
    pub verification_url: String,
    pub expires_in: u32,
    pub interval: u32,
}

#[derive(Debug, Serialize)]
pub struct TokenRequest {
    pub code: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: Option<String>,
    pub grant_type: String, // "authorization_code"
}

#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String, // "bearer"
    pub scope: Option<String>,
}
