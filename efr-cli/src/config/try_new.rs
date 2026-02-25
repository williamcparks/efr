use std::ffi::OsStr;

use crate::config::{ConfigError, EfrConfig};

impl EfrConfig {
    pub fn try_new() -> Result<Self, ConfigError> {
        let mut cwd = match std::env::current_dir() {
            Ok(ok) => ok,
            Err(source) => return Err(ConfigError::Cwd(source)),
        };

        if let Some(arg) = std::env::args().nth(1) {
            cwd.push(arg.as_str());
        };
        if !cwd
            .extension()
            .and_then(OsStr::to_str)
            .map(|v| v == "toml")
            .unwrap_or_default()
        {
            cwd.push("efr.toml");
        }

        Self::try_from_fs(cwd.as_path())
    }
}
