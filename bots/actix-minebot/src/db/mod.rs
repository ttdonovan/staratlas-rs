use color_eyre::Result;
use rusqlite::Connection;

pub struct MinebotDB {
    pub(crate) conn: Connection,
}

impl MinebotDB {
    pub(crate) fn open() -> Result<Self> {
        let conn = rusqlite::Connection::open_in_memory()?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS bot_ops (
                pubkey TEXT PRIMARY KEY,
                state TEXT NOT NULL,
                data TEXT NOT NULL
            )",
            (),
        )?;

        let db = MinebotDB { conn };
        Ok(db)
    }
}
