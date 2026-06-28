mod query;

use time::OffsetDateTime;
use rusqlite::{Connection, params_from_iter};
use directories::ProjectDirs;
use std::fs;
use query::{QueryBuilder};

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

pub struct SearchEntry {
    pub command: String,
    pub path: String,
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

        let creation_stmt = query::new_database();
        conn.execute_batch(
            &creation_stmt
            )?;

        Ok(Database { conn })
    }

    pub fn insert(&self, entry: Entry) -> Result<(), anyhow::Error> {
        let query = query::new_insert();

        self.conn.execute(
            &query,
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

    pub fn search(&self, term: &str, check_time: bool, check_state: bool) -> Result<Vec<SearchEntry>, anyhow::Error> {
        let mut query = QueryBuilder::new();

        let like = format!("%{}%", term);
        query.add_like(&like);

        if check_state {
            query.add_state_check();
        }

        if check_time {
            query.add_time_check();
        }

        let result = query.finalize();
        let mut stmt = self.conn.prepare(&result.0)?;

        let entries = stmt.query_map(params_from_iter(result.1), |row|{
            Ok(SearchEntry {
                command: row.get(0)?,
                path: row.get(1)?,
                timestamp: parse_timestamp(row.get(2)?)?,
            })
        })?
        .collect::<Result<Vec<SearchEntry>, rusqlite::Error>>()?;

        Ok(entries)
    }
}