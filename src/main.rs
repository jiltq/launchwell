//use std::env;
use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::process::Command as Pommand;
extern crate dirs;
extern crate open;

mod maxwell;
mod argemwell;
mod saatana;
mod rufus;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: maxwell::Commands,
}

fn main() {
    let data_dir: PathBuf = dirs::data_local_dir().expect("Local data directory not found").join("VotV");
    let games_dir: PathBuf = dirs::home_dir().expect("Home folder not found.").join("Games/VotV");
    fs::create_dir_all(&games_dir).expect("xd");

    let cli: Cli = Cli::parse();
/*
    let releases = argemwell::get_releases().expect("xd");

    for release in releases {
        println!("{}", release)
    }
*/

    match &cli.command {
        maxwell::Commands::Launch { version } => {
            let exec_dir = games_dir.join(version).join(version).join("WindowsNoEditor").join("VotV.exe");

            let idk = Pommand::new(exec_dir).status().expect("xd");
            println!("'myapp add' was used, name is: {version:?}")
        }
        maxwell::Commands::Assets {} => {
            let assets_dir: PathBuf = data_dir.join("Assets");
            assert!(assets_dir.is_dir(), "VotV/Assets not found");

            open::that(assets_dir).expect("Failed to open VotV/Assets");
        }
        maxwell::Commands::Install {version} => {
            assert!(!games_dir.join(version).is_dir(), "Already installed.");

            let bytes = saatana::download_release(version).expect("xd");
            rufus::install_version(&games_dir, version, bytes).expect("xd");
        }
        maxwell::Commands::Uninstall {version} => {
            rufus::delete_version(&games_dir, version).expect("xd")
        }
        _ => {}
    }
}
