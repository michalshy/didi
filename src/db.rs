use time::OffsetDateTime;
use rusqlite::{Connection, Result};
use directories::ProjectDirs;
use std::fs;


struct Entry {
    session_id: i64,
    command: String,
    path: String,
    exit_code: i32,
    timestamp: OffsetDateTime
}

pub struct Database {
    conn: Connection,
    path: ProjectDirs
}

impl Database {
    pub fn init() -> Result<Database, anyhow::Error> {
        let path = ProjectDirs::from("com", "michalshy", "didi")
            .expect("Could not open project directory");
        fs::create_dir_all(path.data_dir())?;
        let conn = Connection::open(path.data_dir().join("db.sqlite"))
            .expect("Could not open project database");

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS entries(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                session_id INTEGER NOT NULL,
                command TEXT NOT NULL,
                path TEXT NOT NULL,
                exit_code INTEGER NOT NULL,
                timestamp TEXT NOT NULL)"
            )?;

        Ok(Database { conn, path })
    }
}