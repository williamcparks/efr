use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Could not read the directory that contains the config file @ {}", _0.display())]
    NoParent(PathBuf),

    #[error("Failed to read file @ {} due to: {source}", .path.display())]
    ReadFile {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to parse toml file @ {} due to: {source}", .path.display())]
    Toml {
        path: PathBuf,
        #[source]
        source: toml::de::Error,
    },

    #[error("Failed to parse RSA private key @ {} due to: {rsa}", .path.display())]
    RsaPrivateKey { path: PathBuf, rsa: String },

    #[error("Failed to get current working directory due to: {0}")]
    Cwd(#[source] std::io::Error),
}
