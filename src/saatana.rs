use bytes::Bytes;
use reqwest::blocking::{get, ClientBuilder, Response};
use reqwest::Error as R_Error;
use select::document::Document;
use select::node::Node;
use select::predicate::{Class, Name, And, Attr, Predicate};
use std::collections::HashMap;
use std::str::Split;
use std::time::Duration;
use reqwest::{IntoUrl, Error};

// https://github.com/UE4SS-RE/RE-UE4SS/releases/latest

// saatana - web operations and version info

pub static VOTV_7Z_URL: &str = "https://invotek.net/votvarc/{}.7z";
pub static CLIENT_PATIENCE: Duration = Duration::from_secs(60 * 5);

pub fn fetch_doc<T: IntoUrl>(url: T) -> Result<Document, Error> {
    Ok(Document::from(fetch(url)?.text()?.as_str()))
}

pub fn fetch_bytes<T: IntoUrl>(url: T) -> Result<Bytes, Error> {
    fetch(url)?.bytes()
}

fn fetch<T: IntoUrl>(url: T) -> Result<Response, Error> {
    ClientBuilder::new().timeout(CLIENT_PATIENCE).build()?.get(url).send()
}

pub fn has_internet() -> bool {
    ClientBuilder::new().timeout(CLIENT_PATIENCE).build().is_err()
}

pub fn trim_alias(mut alias: String) -> String {
    if let Some(index) = alias.find(|c: char| c.is_digit(10)) {
        alias.drain(..index);
    }
    alias.replace(" ", "")
}

pub fn get_latest_version_id(invotek_doc: &Document) -> Option<String> {
    let release_container: Node = invotek_doc.find(Class("release-container")).next()?;
    let text: String = release_container.find(Name("h1")).next()?.text();
    let splits: Vec<&str> = text.split("/").collect();
    Some(splits.get(1)?.trim().to_string())
}

pub fn get_version_aliases(document: &Document) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();

    let release_containers: Vec<Node> = document.find(Class("release-container")).collect();

    for release_container in release_containers.iter().rev() {
        let header: Node = match release_container.find(Name("h1")).next() {
            Some(header) => header,
            None => break,
        };

        let header_text: String = header.text();

        let header_parts: Vec<&str> = header_text.split("/").collect();

        let soup: String = header_parts[0].replace(" ", "").clone();

        match header_parts.len() {
            1 => {
                map.insert(soup.clone(), soup);
            }
            2 => {
                map.insert(trim_alias(soup.clone()), header_parts[1].replace(" ", ""));
            }
            _ => continue
        }
    }

    return map;
}

pub fn translate_input_to_id(input: &String) -> String {
    let invotek_res: Result<Document, R_Error> = fetch_doc("https://invotek.net/releases.html");
    if invotek_res.is_err() { return input.clone() } // either invotek or the user is offline
    let invotek_doc: Document = invotek_res.expect("Something messed up!");

    let alias_map: HashMap<String, String> = get_version_aliases(&invotek_doc);

    if alias_map.values().any(|v: &String| v == input) {
        input.clone()
    } else if let Some(value) = alias_map.get(input) {
        value.clone()
    } else if let Some(value) = alias_map.get(&trim_alias(input.clone())) {
        value.clone()
    } else {
        input.clone()
    }
}