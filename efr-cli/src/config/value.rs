use std::path::Path;

use rsa::{RsaPrivateKey, pkcs1v15::SigningKey, pkcs8::DecodePrivateKey};
use sha1::Sha1;

use crate::config::{ConfigError, raw::RawEfrConfig};

pub struct EfrConfig {
    pub cert_der: Vec<u8>,
    pub signing_key: SigningKey<Sha1>,
    pub email: Box<str>,
    pub password: Box<str>,
}

impl EfrConfig {
    pub fn try_from_fs(path: &Path) -> Result<Self, ConfigError> {
        let parent = match path.parent() {
            Some(some) => some,
            None => return Err(ConfigError::NoParent(path.to_path_buf())),
        };

        let toml_content = match std::fs::read_to_string(path) {
            Ok(ok) => ok,
            Err(source) => {
                return Err(ConfigError::ReadFile {
                    path: path.to_path_buf(),
                    source,
                });
            }
        };
        let raw: RawEfrConfig = match toml::from_str(toml_content.as_str()) {
            Ok(ok) => ok,
            Err(source) => {
                return Err(ConfigError::Toml {
                    path: path.to_path_buf(),
                    source,
                });
            }
        };

        let cert_path = parent.join(raw.credentials.cert_der);
        let cert_der = match std::fs::read(cert_path.as_path()) {
            Ok(ok) => ok,
            Err(source) => {
                return Err(ConfigError::ReadFile {
                    path: cert_path,
                    source,
                });
            }
        };

        let pkey_path = parent.join(raw.credentials.pkey_pem);
        let pkey_pem = match std::fs::read_to_string(pkey_path.as_path()) {
            Ok(ok) => ok,
            Err(source) => {
                return Err(ConfigError::ReadFile {
                    path: pkey_path,
                    source,
                });
            }
        };

        let rsa_private_key = match RsaPrivateKey::from_pkcs8_pem(pkey_pem.as_str()) {
            Ok(ok) => ok,
            Err(err) => {
                return Err(ConfigError::RsaPrivateKey {
                    path: pkey_path,
                    rsa: err.to_string(),
                });
            }
        };

        Ok(Self {
            cert_der,
            signing_key: SigningKey::new(rsa_private_key),
            email: raw.admin.email.into_boxed_str(),
            password: raw.admin.password.into_boxed_str(),
        })
    }
}
