#[cfg(feature = "cli")]
mod commands;

#[cfg(feature = "cli")]
use std::path::PathBuf;
#[cfg(feature = "cli")]
use clap::{Parser, Subcommand};
#[cfg(feature = "cli")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "cli")]
use crate::commands::SpawnCommand::SpawnSubcommands;

#[cfg(feature = "cli")]
#[derive(Parser, Debug)]
#[command(version, about, long_about = None, author = "JustSunnit", name = "Ag-act")]
struct Args {
    #[arg(short, long, default_value = ".")]
    dir: PathBuf,
    #[command(subcommand)]
    commands: Option<Commands>,
}

#[cfg(feature = "cli")]
#[derive(Subcommand, Debug)]
enum Commands {
    Spawn {
        #[command(subcommand)]
        subcommands: SpawnSubcommands,
    }

}

#[tokio::main]
#[cfg(feature = "cli")]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut config = Ag_act::Config::load()?;

    let cli = Args::parse();

    match &cli.commands {
        None => {},
        Some(Commands::Spawn { subcommands }) => {
            match subcommands {
                SpawnSubcommands::Home { name } => {
                    let mut file_name = name.clone();
                    file_name.push_str(".ag-act");
                    commands::SpawnCommand::home(&mut config, cli.dir.join(file_name)).await?;
                },
                SpawnSubcommands::Project => {
                    commands::SpawnCommand::project(&mut config).await?;
                }
            }
        }
    }

    config.write()?;
    Ok(())
}