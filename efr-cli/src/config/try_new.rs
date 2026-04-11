use std::ffi::OsStr;

use crate::config::{ConfigError, EfrConfig};

impl EfrConfig {
    pub fn try_new() -> Result<Self, ConfigError> {
        let cwd = match std::env::current_dir() {
            Ok(ok) => ok,
            Err(source) => return Err(ConfigError::Cwd(source)),
        };
        let mut path = cwd.clone();

        if let Some(arg) = std::env::args().nth(1) {
            path.push(arg.as_str());
        };
        if !path
            .extension()
            .and_then(OsStr::to_str)
            .map(|v| v == "toml")
            .unwrap_or_default()
        {
            path.push("efr.toml");
        }

        Self::try_from_fs(path.as_path(), cwd)
    }
}
