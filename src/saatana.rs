use bytes::Bytes;
use reqwest::blocking::get;
use reqwest::Error;

pub fn download_release(release: &String) -> Result<Bytes, Error> {
    let response: Bytes = get(&format!("https://invotek.net/votvarc/{}.7z", release))?.bytes()?;
    Ok(response)
}
