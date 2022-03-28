pub mod eqvol;
pub mod extra;
pub mod other;
pub mod regular;

extern crate quick_xml;
use serde::Deserialize;
use std::collections::HashSet;

#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
struct Entry {
    title: String,
    id: String,
    updated: String,
    content: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Feed {
    updated: String,
    #[serde(rename = "entry", default)]
    entries: HashSet<Entry>,
}
