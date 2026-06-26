mod db;
mod parser;

use anyhow::Ok;
use clap::Parser;
use db::{Database, Entry};
use parser::Cli;
use parser::Command::{Log, Search};
use time::OffsetDateTime;

struct App{
    db: Database,
    cli: Cli
}

impl App {
    pub fn init() -> App {
        return App { 
            db: Database::init().expect("Could not initialized database"),
            cli: Cli::parse()
        };
    }

    pub fn execute(&self) -> Result<(), anyhow::Error> {

        match &self.cli.command {
            Log { cmd, cwd, exit, session } => {
                self.log(cmd, cwd, *exit, session)?
            },
            Search { term } => {
                let entries = self.search(term)?;
                for entry in entries {
                    println!(
                        "{}  {}  {}",
                        entry.timestamp, entry.path, entry.command
                    );
                }
            }
        }
        Ok(())
    }

    fn log(&self, cmd: &String, cwd: &String, exit: i32, session: &String) -> Result<(), anyhow::Error> {
        let timestamp = OffsetDateTime::now_utc();

        let entry = Entry { 
            session_id: session.clone(), 
            command: cmd.clone(), 
            path: cwd.clone(), 
            exit_code: exit, 
            timestamp
        };

        self.db.insert(entry)
    }

    fn search(&self, term: &String) -> Result<Vec<Entry>, anyhow::Error> {
        self.db.search(term)
    }
}



fn main() -> Result<(), anyhow::Error> {
    let app = App::init();
    app.execute()
}
