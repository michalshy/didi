use time::OffsetDateTime;
use rusqlite::{Connection, Result};

struct Entry {
    session_id: i64,
    command: String,
    path: String,
    exit_code: i32,
    timestamp: OffsetDateTime
}

pub struct Database {
    conn: Connection
}

impl Database {
    pub fn init() -> Database {
        let conn = Connection::open_in_memory().expect("Could not open database");
        Database { conn }
    }
}