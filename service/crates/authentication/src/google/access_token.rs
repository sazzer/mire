use serde::Deserialize;

/// Representation of an Access Token received from Google
#[derive(Debug, Deserialize)]
pub struct AccessToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i32,
    pub id_token: String,
}
