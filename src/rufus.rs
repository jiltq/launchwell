use bytes::Bytes;
use sevenz_rust::{decompress, Error as Zerror};
use std::fs::{read_dir, remove_dir_all};
use std::io::{Cursor, Error as IO_Error};
use std::path::PathBuf;

// rufus - for file operations

pub fn delete_version(games_dir: &PathBuf, version_id: &String) -> Result<(), IO_Error> {
    let release_dir: PathBuf = games_dir.join(version_id);

    let exec_dir: PathBuf = release_dir.join(version_id).join("WindowsNoEditor/VotV.exe");
    assert!(exec_dir.is_file(), "No installation found with that name."); // O_O''

    remove_dir_all(release_dir)?;

    Ok(())
}

pub fn install_version(games_dir: &PathBuf, version_id: &String, bytes: Bytes) -> Result<(), Zerror> {
    decompress(Cursor::new(bytes), games_dir.join(version_id))?;
    Ok(())
}

pub fn list_installations(votv_dir: &PathBuf) -> Result<Vec<String>, IO_Error> {
    let installations: Vec<String> = read_dir(votv_dir)?
        .filter_map(Result::ok)
        .filter(|e| e.path().is_dir())
        .map(|e| e.file_name().into_string().unwrap())
        .collect::<Vec<_>>();
    Ok(installations)
}
