use clap::Subcommand;
use clap::Parser;

#[derive(Subcommand)]
pub enum Commands {
    /// Launches a specific version or latest
    Launch {
        /// version to launch
        version: String
        //version: Option<String>
    },
    /// Installs a version of the game
    Install {
        version: String
    },
    /// Uninstalls a version of the game
    Uninstall {
        version: String
    },
    /// Fetches information about version(s)
    Info {
        version: Option<String>
    },
    /// Opens the "Assets" folder
    Assets {},
    /// Lists installed versions
    Installed {},
    Modwiki {},

    UE4ss {
        /// The version of the VotV installation to install UE4SS to
        votv_version: String
    }
}