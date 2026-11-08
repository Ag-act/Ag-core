mod gui;

use std::path::PathBuf;
#[cfg(feature = "cli")]
use clap::{Parser, Subcommand};
use color_eyre::eyre;
use serde::{Deserialize, Serialize};

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
        subcommands: SpawnSubcommands
    }

}
#[cfg(feature = "cli")]
#[derive(Subcommand, Debug)]
enum SpawnSubcommands {
    Home,
    Project
}

#[tokio::main]
#[cfg(feature = "cli")]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let cli = Args::parse();

    match &cli.commands {
        None => {
            spawn_gui()?;
        },
        Some(Commands::Spawn {subcommands}) => {
            match subcommands {
                SpawnSubcommands::Home => {
                    let set_def = asking::yn()
                        .message("Do you want this to be the default")
                        .default_value(true)
                        .ask();

                    match set_def.await {
                        Ok(true) => {

                        }
                        Ok(false) => {

                        }

                        Err(e) => {
                            eyre::bail!("{}", e);
                        }
                    }
                },
                SpawnSubcommands::Project => {}
            }
        }
    }

    Ok(())
}

#[cfg(feature = "cli")]
fn spawn_gui() -> std::io::Result<()>  {
    ratatui::run(gui::app::tui)?;
    Ok(())
}

#[cfg(not(feature = "cli"))]
fn main() {
    println!("CLI Not active");
}