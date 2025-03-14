/*
 *  Copyright (C) 2025 {{username}}
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU Affero General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU Affero General Public License for more details.
 *
 *  You should have received a copy of the GNU Affero General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

 use crate::error::{{crate_name | upper_camel_case}}Result;
use clap::Parser;
use gethostname::gethostname;
use serde::Deserialize;
use std::{
    net::{IpAddr, Ipv4Addr},
    path::{Path, PathBuf},
};
use tokio::fs;
use tracing::{info, warn};

#[derive(Parser)]
#[command(author, version, about, long_about)]
pub struct Args {
    /// Path to config file
    #[arg(short, long, env = "{{crate_name | shouty_snake_case}}_CONFIG")]
    config: Option<PathBuf>,

    /// Web server bind address
    #[arg(
        short,
        long,
        env = "{{crate_name | shouty_snake_case}}_WEB_SERVER_BIND_ADDRESS"
    )]
    bind_address: Option<IpAddr>,

    /// Web server port
    #[arg(
        short,
        long,
        env = "{{crate_name | shouty_snake_case}}_WEB_SERVER_PORT"
    )]
    port: Option<u16>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebServerConfig {
    pub bind_address: IpAddr,
    pub port: u16,
}

impl Default for WebServerConfig {
    fn default() -> Self {
        Self {
            bind_address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port: 3000,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    pub name: String,
    pub instance: InstanceConfig
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            name: "{{crate_name | kebab_case}}".to_owned(),
            instance: InstanceConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstanceConfig {
    pub name: String,
}

impl Default for InstanceConfig {
    fn default() -> Self {
        Self {
            name: gethostname().to_string_lossy().to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Config {
    #[serde(default = "AppConfig::default")]
    pub app: AppConfig,
    #[serde(default = "WebServerConfig::default")]
    pub webserver: WebServerConfig,
}

impl Config {
    pub async fn load() -> {{crate_name | upper_camel_case}}Result<Config> {
        let args = Args::parse();

        info!("Loading config …");

        let mut config = Config::load_from_file(args.config.as_deref()).await?;

        config.merge(args)?;

        Ok(config)
    }

    async fn load_from_file(path: Option<&Path>) -> {{crate_name | upper_camel_case}}Result<Config> {
        match path {
            Some(path) => {
                let content = fs::read_to_string(&path).await?;
                let config = serde_yaml::from_str(&content)?;
                info!("Config loaded from {}", path.to_string_lossy());
                Ok(config)
            }
            None => {
                let path = if cfg!(debug_assertions) {
                    let it = "./config-dev.yaml";
                    warn!("No config file specified, using {it}");
                    it
                } else {
                    let it = "/etc/{{crate_name | kebab_case}}/config.yaml";
                    warn!("No config file specified, using {it}");
                    it
                };
                match fs::read_to_string(path).await {
                    Ok(it) => {
                        let config = serde_yaml::from_str(&it)?;
                        info!("Config loaded from {path}");
                        Ok(config)
                    }
                    Err(_) => {
                        warn!("Could not read config file {path}, using default config.");
                        Ok(Config::default())
                    }
                }
            }
        }
    }

    fn merge(&mut self, args: Args) -> {{crate_name | upper_camel_case}}Result<()> {
        if let Some(it) = args.bind_address {
            self.webserver.bind_address = it
        }

        if let Some(it) = args.port {
            self.webserver.port = it
        }

        Ok(())
    }
}
