use clap::{Parser, Subcommand};


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli
{
    #[command(subcommand)]
    command: Command
}

#[derive(Subcommand, Debug)]
enum Command {
    Log {
        #[arg(long)]
        cmd: String,

        #[arg(long)]
        cwd: String,

        #[arg(long)]
        exit: String,
    },
    Search {
        term: String
    }
}