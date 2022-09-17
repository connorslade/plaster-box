use std::path::Path;
use std::str::FromStr;

use simple_config_parser::Config as Cfg;

pub struct Config {
    pub host: String,
    pub port: u16,

    pub data_limit: usize,
    pub database_file: String,
}

impl Config {
    pub fn new<T: AsRef<Path>>(file: T) -> Self {
        let cfg = Cfg::new().file(file).unwrap();

        Self {
            host: get_config(&cfg, "host"),
            port: get_config(&cfg, "port"),

            data_limit: get_config(&cfg, "data_limit"),
            database_file: get_config(&cfg, "database_file"),
        }
    }
}

fn get_config<T: FromStr>(cfg: &Cfg, name: &str) -> T {
    cfg.get(name)
        .unwrap_or_else(|_| panic!("Error getting `{}` from Config", name))
}
