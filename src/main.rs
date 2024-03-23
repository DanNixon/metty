mod commands;
mod formatting;

use cadmium_yellow::Client;
use clap::{Args, CommandFactory, Parser, Subcommand};
use clap_complete::Shell;

type Result<T> = std::result::Result<T, String>;

/// A command line tool for showing details about the Tyne and Wear Metro.
#[derive(Parser)]
#[command(name = "metty", version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    cmd: SubCommand,
}

#[derive(Subcommand)]
enum SubCommand {
    /// List all stations, their codes and platforms
    Stations,

    /// List stations on a specific line
    Line(LineArgs),

    /// List arrival times for a given station and platform
    Times(TimesArgs),

    /// Generate shell completions
    Completions(CompletionsArgs),
}

#[derive(Args)]
struct LineArgs {
    /// The name of the line to show (either "green" or "yellow")
    line: String,
}

#[derive(Args)]
struct TimesArgs {
    /// The three letter identifier/code of the station to query
    #[clap(env = "METTY_STATION")]
    station: String,

    /// The platform number to query
    #[clap(env = "METTY_PLATFORM")]
    platform: i64,
}

#[derive(Args)]
struct CompletionsArgs {
    /// The shell to generate completions for
    shell: Shell,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Cli::parse();

    let client = Client::default();

    match opts.cmd {
        SubCommand::Stations => crate::commands::print_stations(&client).await,
        SubCommand::Line(args) => crate::commands::print_line(&client, args).await,
        SubCommand::Times(args) => crate::commands::print_times(&client, args).await,
        SubCommand::Completions(args) => {
            let mut cmd = Cli::command();
            let name = cmd.get_name().to_string();
            clap_complete::generate(args.shell, &mut cmd, name, &mut std::io::stdout());
            Ok(())
        }
    }
}
