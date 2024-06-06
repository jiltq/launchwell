use reqwest::blocking::{get, Response};
use reqwest::Error;
use select::document::Document;
use select::node::Node;
use select::predicate::{Class, Name};
use std::collections::HashMap;

fn get_invotek_doc() -> Result<Document, Error> {
    let html: String = get("https://invotek.net/releases.html")?.text()?;
    let document: Document = Document::from(html.as_str());
    Ok(document)
}

pub fn get_releases() -> Result<Vec<String>, Error> {
    let mut releases: Vec<String> = Vec::new();

    let document: Document = get_invotek_doc()?;

    for release_container in document.find(Class("release-container")) {
        let download_link: Node = release_container.find(Class("download-link")).next().expect("xd");
        let hyperlink: Node = download_link.find(Name("a")).next().expect("xd");
        let href: &str = hyperlink.attr("href").expect("xd");

        releases.push(href.trim_start_matches("/releases/").trim_end_matches(".7z").to_string());
    }

    Ok(releases)
}

/* 
pub fn fetch_latest_release() -> Result<(Node), Error> {
    let html: String = get("https://invotek.net/releases.html")?.text()?;

    let document: Document = Document::from(html.as_str());

    let idk = document.find(Class("release-container")).next().expect("lol");

    Ok(idk)
}
*/
/*

pub struct Version {
    name: String,
    id: String,
}

pub fn get_version_info(version: &String) {
    let html: String = get("https://invotek.net/releases.html")?.text()?;

    let document: Document = Document::from(html.as_str());
    
    for release_container in document.find(Class("release-container")) {
        let download_link: Node = release_container.find(Class("download-link")).next().expect("xd");
        let hyperlink: Node = download_link.find(Name("a")).next().expect("xd");
        let href: &str = hyperlink.attr("href").expect("xd");

        releases.push(href.trim_start_matches("/releases/").trim_end_matches(".7z").to_string());
    }
}

*/


/* 
pub fn get_infos() -> Result<Find, Error> {
    let html: String = get("https://invotek.net/releases.html")?.text()?;

    let response: Response = get("https://invotek.net/releases.html")?;
    let html: String = response.text()?;

    let document: Document = Document::from(html.as_str());

    document.find(Class("release-container"))?
}



pub fn get_version_info(version: &String) -> Result<Node, Error> {
    let response: Response = get("https://invotek.net/releases.html")?;
    let html: String = response.text()?;

    let document: Document = Document::from(html.as_str());

    Ok(())
}

*/