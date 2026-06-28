use clap::{Parser, Subcommand};


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli
{
    #[command(subcommand)]
    pub command: Command
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Log {
        #[arg(long)]
        cmd: String,

        #[arg(long)]
        cwd: String,

        #[arg(long)]
        exit: i32,

        #[arg(long)]
        session: String
    },
    Search {
        term: String
    }
}