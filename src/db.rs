use time::OffsetDateTime;
use rusqlite::{Connection};
use directories::ProjectDirs;
use std::fs;

fn parse_timestamp(raw: String) -> rusqlite::Result<OffsetDateTime> {
    OffsetDateTime::parse(&raw, &time::format_description::well_known::Rfc3339)
        .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))
}

pub struct Entry {
    pub session_id: String,
    pub command: String,
    pub path: String,
    pub exit_code: i32,
    pub timestamp: OffsetDateTime
}

pub struct Database {
    conn: Connection,
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
                session_id TEXT NOT NULL,
                command TEXT NOT NULL,
                path TEXT NOT NULL,
                exit_code INTEGER NOT NULL,
                timestamp TEXT NOT NULL)"
            )?;

        Ok(Database { conn })
    }

    pub fn insert(&self, entry: Entry) -> Result<(), anyhow::Error> {
        self.conn.execute(
            "INSERT INTO entries(session_id, command, path, exit_code, timestamp)
            VALUES (?1, ?2, ?3, ?4, ?5)",
            (
                &entry.session_id,
                &entry.command,
                &entry.path,
                &entry.exit_code,
                entry.timestamp.format(&time::format_description::well_known::Rfc3339)
                    .expect("failed to format timestamp"),
            ),
        )?;
        Ok(())
    }

    pub fn search(&self, term: &str) -> Result<Vec<Entry>, anyhow::Error> {
        let mut stmt = self.conn.prepare(
            "SELECT session_id, command, path, exit_code, timestamp
            FROM entries
            WHERE command LIKE ?1
            ORDER BY timestamp DESC"
        )?;
        let pattern = format!("%{}%", term);

        let entries = stmt.query_map([pattern], |row|{
            Ok(Entry {
                session_id: row.get(0)?,
                command: row.get(1)?,
                path: row.get(2)?,
                exit_code: row.get(3)?,
                timestamp: parse_timestamp(row.get(4)?)?,
            })
        })?
        .collect::<Result<Vec<Entry>, rusqlite::Error>>()?;

        Ok(entries)
    }
}