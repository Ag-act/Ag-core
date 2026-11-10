#![cfg(feature = "cli")]

use std::path::{PathBuf};
use clap::Subcommand;
use color_eyre::config;
use Ag_act::{Config, DataHandler};

#[derive(Subcommand, Debug)]
pub(crate) enum SpawnSubcommands {
    Home {
        name: String,
    },
    Project
}

pub(crate) async fn home(config : &mut Config, dir : PathBuf) -> color_eyre::Result<()> {
    let set_def = asking::yn()
        .message("Do you want to add this to your homes (y/n) - ")
        .default_value(true)
        .ask();

    match set_def.await {
        Ok(b) => {
            match config.get_version() {
                Ag_act::AgactVersions::V1 => {
                    DataHandler::v1().write(dir.clone())?;
                }
            }

            if b {
                let mut homes = config.get_homes();
                homes.push(dir);
                config.set_homes(homes.clone());
            }

            Ok(())
        }

        Err(e) => {
            color_eyre::eyre::bail!("{}", e);
        }
    }
}

pub(crate) async fn project(config: &mut Config) -> color_eyre::Result<()> {
    let set_def = asking::yn()
        .message("Do you want to add this to your projects (y/n) - ")
        .default_value(true)
        .ask();

    match set_def.await {
        Ok(b) => {
            color_eyre::eyre::bail!("{}", "This is not yet implemented");
        }
        Err(e) => {
            color_eyre::eyre::bail!("{}", e);
        }
    }
}