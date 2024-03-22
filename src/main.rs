mod commands;
mod formatting;

use cadmium_yellow::Client;
use clap::{Args, Parser, Subcommand};

type Result<T> = std::result::Result<T, String>;

/// A command line tool for showing details about the Tyne and Wear Metro.
#[derive(Debug, Parser)]
struct Cli {
    #[clap(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, Subcommand)]
enum SubCommand {
    /// List all stations, their codes and platforms
    Stations,

    /// List stations on a specific line
    Line(LineArgs),

    /// List arrival times for a given station and platform
    Times(TimesArgs),
}

#[derive(Debug, Args)]
struct LineArgs {
    line: String,
}

#[derive(Debug, Args)]
struct TimesArgs {
    #[clap(env = "METTY_STATION")]
    station: String,

    #[clap(env = "METTY_PLATFORM")]
    platform: i64,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Cli::parse();

    let client = Client::default();

    match opts.cmd {
        SubCommand::Stations => crate::commands::print_stations(&client).await,
        SubCommand::Line(args) => crate::commands::print_line(&client, args).await,
        SubCommand::Times(args) => crate::commands::print_times(&client, args).await,
    }
}
