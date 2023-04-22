use std::env;

use parking_lot::{Mutex, MutexGuard};
use rusqlite::Connection;

use crate::{config::Config, database::Database};

pub struct App {
    pub config: Config,
    pub database: Mutex<Connection>,
}

impl App {
    pub fn new() -> Self {
        let cfg_path = env::vars()
            .find(|x| x.0 == "config")
            .map(|x| x.1)
            .unwrap_or_else(|| "./data/config.cfg".to_owned());
        let cfg = Config::new(cfg_path);

        let db = Connection::open(&cfg.database_file).unwrap();
        db.init().unwrap();

        Self {
            config: cfg,
            database: Mutex::new(db),
        }
    }

    pub fn db(&self) -> MutexGuard<Connection> {
        self.database.lock()
    }
}
