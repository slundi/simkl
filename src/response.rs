use serde::de::DeserializeOwned;
use serde_json::Value;

pub struct SimklResponse {
    pub status_code: u16,
    pub headers: std::collections::HashMap<String, String>,
    pub body: String,
}

impl SimklResponse {
    pub fn new(
        status_code: u16,
        headers: std::collections::HashMap<String, String>,
        body: String,
    ) -> Self {
        Self {
            status_code,
            headers,
            body,
        }
    }

    pub fn json<T: DeserializeOwned>(&self) -> crate::error::Result<T> {
        Ok(serde_json::from_str(&self.body)?)
    }

    pub fn json_value(&self) -> crate::error::Result<Value> {
        Ok(serde_json::from_str(&self.body)?)
    }

    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.status_code)
    }

    pub fn is_client_error(&self) -> bool {
        (400..500).contains(&self.status_code)
    }

    pub fn is_server_error(&self) -> bool {
        (500..600).contains(&self.status_code)
    }
}
