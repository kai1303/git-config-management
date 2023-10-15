use serde_derive::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub host: String,
    pub host_name: String,
    pub user: String,
    pub identity_file: String,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.config_to_string())    
    }
}

impl Config {
    pub fn config_to_string(&self) -> String {
    format!(r#"
Host {host}
    HostName {host_name}
    USer {user}
    IdentityFile {identity_file}"#, host = self.host, host_name = self.host_name, user = self.user, identity_file = self.identity_file)
    }
}

