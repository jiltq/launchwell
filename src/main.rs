//use std::env;
use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::process::Command as Pommand;
extern crate dirs;
extern crate open;

mod maxwell;
mod saatana;
mod rufus;
mod argemwell;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: maxwell::Commands,
}

fn main() {
    //std::env::set_var("RUST_BACKTRACE", "full");
    let data_dir: PathBuf = dirs::data_local_dir().expect("Local data directory not found").join("VotV");
    let games_dir: PathBuf = dirs::home_dir().expect("Home folder not found.").join("Games/VotV");
    fs::create_dir_all(&games_dir).expect("xd");

    let cli: Cli = Cli::parse();

    match &cli.command {
        maxwell::Commands::Launch { version } => {
            let id: &String = &saatana::translate_input_to_id(version);
            let exec_dir = games_dir.join(id).join(id).join("WindowsNoEditor").join("VotV.exe");
            assert!(exec_dir.is_file(), "Version not installed.");
            Pommand::new(exec_dir).status().expect("xd");
        }
        maxwell::Commands::Assets {} => {
            let assets_dir: PathBuf = data_dir.join("Assets");
            assert!(assets_dir.is_dir(), "VotV/Assets not found");
            open::that(assets_dir).expect("Failed to open VotV/Assets");
        }
        maxwell::Commands::Install {version} => {
            let id: &String = &saatana::translate_input_to_id(version);
            assert!(!games_dir.join(id).is_dir(), "Version is already installed.");
            let bytes = saatana::fetch_bytes(&format!("https://invotek.net/votvarc/{}.7z", version)).expect("Failed to download version.");
            rufus::extract_7z_to_dir(&games_dir.join(id), bytes).expect("xd");
            rufus::mark_dir_as_votv(&games_dir.join(id)).expect("xd");
        }
        maxwell::Commands::Uninstall {version} => {
            let id: &String = &saatana::translate_input_to_id(version);
            rufus::delete_version(&games_dir, id).expect("xd")
        }
        maxwell::Commands::Modwiki {} => {
            open::that(argemwell::MOD_WIKI_URL).expect("Failed to open modwiki");
        }
        maxwell::Commands::UE4ss { votv_version} => {
            let dl_url: String = argemwell::fetch_ue4ss_url();
            let bytes = saatana::fetch_bytes(&dl_url).expect("xd");
            let exec_dir = games_dir.join(votv_version).join(votv_version).join(rufus::UE4SS_INSTALL_DIR);
            rufus::extract_zip_to_dir(&exec_dir, bytes).expect("noo");
        }
        _ => {}
    }
}
