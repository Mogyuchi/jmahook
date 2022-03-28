extern crate quick_xml;
extern crate serde;

use crate::feed::{Entry, Feed};
use crate::webhook::Webhook;
use quick_xml::de::from_str;
use std::collections::hash_map::RandomState;
use std::collections::hash_set::Difference;
use std::fs::{self, File};
use std::io::Write;

pub async fn regular() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://www.data.jma.go.jp/developer/xml/feed/regular.xml")
        .await?
        .text()
        .await?;
    let doc: Feed = from_str(&resp)?;
    let old_doc: Feed = from_str(&load().unwrap_or_default())?;
    if doc.updated != old_doc.updated {
        save(resp).unwrap();
        send(doc.entries.difference(&old_doc.entries)).await;
    }

    Ok(())
}

async fn send(entries: Difference<'_, Entry, RandomState>) {
    for entry in entries {
        let content = Webhook {
            content: Some(format!("[{}]({})", entry.content, entry.id)),
            username: Some(entry.title.clone()),
            ..Default::default()
        };
        crate::webhook::send(content).await;
    }
}

fn load() -> Result<String, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("./data/regular.xml").expect("could not read file");
    Ok(content)
}

fn save(xml: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create("./data/regular.xml")?;
    writeln!(file, "{}", xml)?;
    file.flush()?;
    Ok(())
}
