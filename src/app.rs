use std::env;

use parking_lot::Mutex;
use rusqlite::Connection;

use crate::config::Config;

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
        db.pragma_update(None, "journal_mode", "WAL").unwrap();
        db.pragma_update(None, "synchronous", "NORMAL").unwrap();
        db.execute(include_str!("./sql/create_bins.sql"), [])
            .unwrap();

        Self {
            config: cfg,
            database: Mutex::new(db),
        }
    }
}
