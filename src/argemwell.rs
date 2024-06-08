use bytes::Bytes;
use reqwest::blocking::get;
use reqwest::Error as R_Error;
use select::document::Document;
use select::node::Node;
use select::predicate::{Class, Name, And, Attr, Predicate};
use std::collections::HashMap;
use reqwest::IntoUrl;

// argemwell - modding stuffs

pub static MOD_WIKI_URL: &str = "https://modding.ariral.space";
pub static UE4SS_MAIN_URL: &str = "https://github.com/UE4SS-RE/RE-UE4SS/releases/latest";

/*
    MOD_AUTHOR-MOD_NAME-#.#.#.zip
        L manifest.json
        L README.md
        L icon.png
        L mod
            L enabled.txt
            L scripts
                L main.lua
            L dll
                L main.dll
        L pak
            L MOD_NAME.pak
        L cfg
            L MOD_AUTHOR-MOD_NAME-cfg.json
*/

pub fn fetch_ue4ss_url() -> String {
    let ue4ss_doc: Document = crate::saatana::fetch_doc("https://github.com/UE4SS-RE/RE-UE4SS/releases/latest").expect("xd");

    let predicate = Attr("id", "repo-content-pjax-container")
        .descendant(Name("div"))
        .descendant(Name("nav"))
        .descendant(Name("ol"))
        .descendant(Name("li")
            .and(Class("breadcrumb-item"))
            .and(Class("breadcrumb-item-selected")))
        .descendant(Name("a"));

    let something = ue4ss_doc.find(predicate).next().expect("xd");
    let binding = something.text();
    let version = binding.trim();

    format!("https://github.com/UE4SS-RE/RE-UE4SS/releases/download/{}/zDEV-UE4SS_{}.zip", version, version)
}