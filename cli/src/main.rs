use clap::{Parser, Subcommand};
use trex_parser::{error::Result, Regex};

const _TOML: &'static str = include_str!("../Cargo.toml");

/// Terminal Regular Expression
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Parses a regular expression
    Parse {
        expression: String,
        #[arg(short, long)]
        ignore_case: bool,
        #[arg(short, long)]
        multiline: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Parse { expression, .. } => {
            let re: Regex = expression.parse()?;
            println!("{re}");
            Ok(())
        }
    }
}
