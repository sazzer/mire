#[derive(Debug)]
pub struct Config {
    /// The URL to connect to the database
    pub database_url: String,

    /// The configuration for authentication via Google
    pub google_config: Option<mire_authentication::google::Config>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database_url: "".to_owned(),
            google_config: None,
        }
    }
}
