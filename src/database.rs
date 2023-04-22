use rusqlite::{params, Connection};
use uuid::Uuid;

pub trait Database {
    // == Base ==
    fn init(&self) -> anyhow::Result<()>;
    fn cleanup(&self) -> anyhow::Result<()>;

    // == Bins ==
    fn create_bin(&self, name: &str, body: &str, hidden: bool) -> anyhow::Result<Uuid>;
    fn get_bin(&self, uuid: Uuid) -> anyhow::Result<Option<Bin>>;
}

impl Database for Connection {
    fn init(&self) -> anyhow::Result<()> {
        self.pragma_update(None, "journal_mode", "WAL")?;
        self.pragma_update(None, "synchronous", "NORMAL")?;
        self.execute(include_str!("./sql/create_bins.sql"), [])?;
        Ok(())
    }

    fn cleanup(&self) -> anyhow::Result<()> {
        self.pragma_update(None, "wal_checkpoint", "TRUNCATE")?;
        Ok(())
    }

    fn create_bin(&self, name: &str, body: &str, hidden: bool) -> anyhow::Result<Uuid> {
        let uuid = Uuid::new_v4();
        self.execute(
            include_str!("sql/insert_bin.sql"),
            params![uuid.to_string(), body, name, hidden as u8],
        )?;

        Ok(uuid)
    }

    fn get_bin(&self, uuid: Uuid) -> anyhow::Result<Option<Bin>> {
        let (body, name) = match self.query_row(
            "SELECT data, name FROM bins WHERE uuid = ?",
            [uuid.to_string()],
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
        ) {
            Ok(i) => i,
            Err(rusqlite::Error::QueryReturnedNoRows) => return Ok(None),
            e @ _ => e?,
        };

        Ok(Some(Bin { name, body }))
    }
}

pub struct Bin {
    pub name: String,
    pub body: String,
}
