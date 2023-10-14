use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub host: String,
    pub host_name: String,
    pub user: String,
    pub identity_file: String,
}


