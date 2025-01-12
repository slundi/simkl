//! PIN for devices with limited UI

use serde::Deserialize;

use crate::API_URL;

/// Request a device code URL
pub fn get_pin_request(client_id: String, redirect_url: String) -> String {
    let mut result = String::from(API_URL);
    result.push_str("oauth/pin?client_id=");
    result.push_str(&client_id);
    result.push_str("&redirect=");
    result.push_str(&redirect_url);
    result
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PinResponse {
    pub result: String,
    pub device_code: String,
    pub user_code: String,
    pub verification_url: String,
    pub expires_in: u16,
    pub interval: u8,
}

/// Responses examples:
/// ```json
/// {'result': 'KO', 'message': 'Authorization pending'}
/// ```
/// ```json
/// {'result': 'KO', 'message': 'Slow down'}
/// ```
/// ```json
/// {'result': 'KO', 'access_token': ACCESS_TOKEN}
/// ```
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CodeResponse {
    pub result: String,
    pub message: Option<String>,
    pub access_token: Option<String>,
}

pub fn get_code_status_request(user_code: String, client_id: String) -> String {
    let mut result = String::from(API_URL);
    result.push_str("oauth/pin/");
    result.push_str(&user_code);
    result.push_str("?client_id=");
    result.push_str(&client_id);
    result
}
