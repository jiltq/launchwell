use bytes::Bytes;
use sevenz_rust::{decompress, Error as Zerror};
use std::fs::{read_dir, remove_dir_all, File, create_dir_all};
use std::io::{Cursor, Error as IO_Error, Write};
use std::path::PathBuf;
use zip::read::ZipArchive;
use zip::result::ZipError;

// rufus - for file operations

pub static UE4SS_INSTALL_DIR: &str = "WindowsNoEditor/VotV/Binaries/Win64";
pub static VOTV_VERSION_MARKER: &str = ".isvotv";
pub static VOTV_FRONTEND: &str = "WindowsNoEditor/VotV.exe";

pub fn delete_version(games_dir: &PathBuf, version_id: &String) -> Result<(), IO_Error> {
    let release_dir: PathBuf = games_dir.join(version_id);

    assert!(is_dir_votv(&release_dir), "No installation found with that name."); // O_O''

    remove_dir_all(release_dir)?;

    Ok(())
}

pub fn extract_7z_to_dir(dest: &PathBuf, bytes: Bytes) -> Result<(), Zerror> {
    decompress(Cursor::new(bytes), dest)
}

pub fn extract_zip_to_dir(dest: &PathBuf, bytes: Bytes) -> Result<(), ZipError> {
    ZipArchive::new(Cursor::new(bytes))?.extract(dest)
}

pub fn is_dir_votv(dir: &PathBuf) -> bool {
    dir.join(VOTV_VERSION_MARKER).is_file()
}

pub fn mark_dir_as_votv(dir: &PathBuf) -> Result<File, IO_Error> {
    File::create(dir.join(VOTV_VERSION_MARKER))
}

pub fn list_installations(votv_dir: &PathBuf) -> Result<Vec<String>, IO_Error> {
    let installations: Vec<String> = read_dir(votv_dir)?
        .filter_map(Result::ok)
        .filter(|e| e.path().is_dir())
        .map(|e| e.file_name().into_string().unwrap())
        .collect::<Vec<_>>();
    Ok(installations)
}
