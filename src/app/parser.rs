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
        #[arg(short, long)]
        cmd: String,

        #[arg(short, long)]
        cwd: String,

        #[arg(short, long)]
        exit: i32,

        #[arg(short, long)]
        session: String,
    },
    Search {
        #[arg(short, long)]
        term: String,

        #[arg(short, long)]
        since: Option<String>,
        
        #[arg(short, long)]
        state: Option<i32>
    }
}