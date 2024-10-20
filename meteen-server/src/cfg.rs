use color_eyre::{
    eyre::{eyre, Context},
    Result,
};
use std::{
    env::var,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    path::Path,
    str::FromStr,
};

#[derive(Debug, Clone)]
pub struct Config {
    pub address: IpAddr,
    pub port: u16,
    pub db_name: String,
    pub db_pass: String,
    pub db_user: String,
    pub db_host: String,
    pub data_dir: std::path::PathBuf,
}

impl Config {
    pub fn from_env() -> Result<Config> {
        let address: IpAddr = {
            let addr_str = var("METEEN_ADDRESS").unwrap_or("127.0.0.1".into());
            match (Ipv4Addr::from_str(&addr_str), Ipv6Addr::from_str(&addr_str)) {
                (Ok(addr), _) => addr.into(),
                (_, Ok(addr)) => addr.into(),
                _ => return Err(eyre!("METEEN_ADDRESS is not a valid ipv4 or ipv6 address")),
            }
        };

        let port_str = var("METEEN_PORT").unwrap_or("3332".into());
        let port: u16 = port_str
            .parse()
            .context("METEEN_PORT is not a valid port")?;

        let db_name = empty_string_is_none(var("METEEN_DB_NAME").ok())
            .unwrap_or("meteen".into());
        let db_pass = empty_string_is_none(var("METEEN_DB_PASS").ok())
            .unwrap_or("meteen".into());
        let db_user = empty_string_is_none(var("METEEN_DB_USER").ok())
            .unwrap_or("meteen".into());
        let db_host = empty_string_is_none(var("METEEN_DB_HOST").ok())
            .unwrap_or("127.0.0.1".into());

        let data_dir = match var("METEEN_DATA_DIR") {
            Ok(path_str) => std::path::PathBuf::from_str(&path_str)
                .context("METEEN_DATA_DIR is not a valid path")?,
            Err(_) => {
                let dirs = directories::BaseDirs::new()
                    .ok_or(eyre!("Could not determine base directory"))?;
                dirs.data_dir().join("/meteen-data")
            }
        };

        Ok(Config {
            address,
            port,
            db_name,
            db_pass,
            db_user,
            db_host,
            data_dir,
        })
    }
}

fn empty_string_is_none(str: Option<String>) -> Option<String> {
    match str {
        Some(str) if str == "" => Some("127.0.0.1".into()),
        Some(str) => Some(str.into()),
        _ => None
    }
}
