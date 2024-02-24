use crate::trigger::TriggerKind;
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, io::ErrorKind, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub applications: HashMap<String, AppConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub bin_path: String,
    pub trigger: TriggerKind,
}

impl Config {
    pub fn load() -> Result<Option<Self>> {
        let config_paths = [PathBuf::new()
            .join(env!("HOME"))
            .join(".config/appvis/config.toml")];

        for config_path in config_paths {
            let toml_str = match fs::read_to_string(config_path) {
                Ok(f) => f,
                Err(e) => match e.kind() {
                    ErrorKind::NotFound => continue,
                    _ => bail!(e),
                },
            };
            return match toml::from_str(&toml_str) {
                Ok(config) => Ok(Some(config)),
                Err(e) => bail!(e),
            };
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, time::Duration};

    use super::{AppConfig, Config};
    use crate::trigger::{
        network::WifiConnected, system::AfterLaunchSchedulerLaunched, TriggerKind,
    };

    #[test]
    fn test_serde() {
        let config = &Config {
            applications: HashMap::from_iter([
                (
                    "bash".to_string(),
                    AppConfig {
                        bin_path: "/bin/bash".to_string(),
                        trigger: TriggerKind::WifiConnected(WifiConnected {
                            interval: Duration::from_secs(1),
                        }),
                    },
                ),
                (
                    "zsh".to_string(),
                    AppConfig {
                        bin_path: "/bin/zsh".to_string(),
                        trigger: TriggerKind::AfterLaunchSchedulerLaunched(
                            AfterLaunchSchedulerLaunched {},
                        ),
                    },
                ),
            ]),
        };
        let toml_str = toml::to_string(config).unwrap();
        println!("{}", &toml_str);
        let config: Config = toml::from_str(&toml_str).unwrap();
        assert_eq!(config.applications.len(), 2);
    }
}
