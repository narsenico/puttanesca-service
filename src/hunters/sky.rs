use super::{Match, MatchDay};
use crate::Result;
use async_trait::async_trait;
use reqwest::Client;
use select::{
    document::Document,
    node::Node,
    predicate::{Attr, Class, Name, Predicate},
};
use std::time::Duration;

pub struct SkyHunter {
    http_client: Client,
}

impl SkyHunter {
    pub fn new() -> Result<Self> {
        let http_timeout = Duration::from_secs(6);
        let http_client = Client::builder().timeout(http_timeout).build()?;

        Ok(Self { http_client })
    }
}

#[async_trait]
impl super::Hunter for SkyHunter {
    fn name(&self) -> String {
        String::from("Test Hunter")
    }

    async fn find_matches(&self) -> crate::Result<Vec<Match>> {
        let url = "https://sport.sky.it/calcio/serie-a/calendario-risultati#giornata-1";
        let http_res = self.http_client.get(url).send().await?.text().await?;

        let document = Document::from(http_res.as_str());
        // div[data-intersect]
        let divs = document.find(Name("div").and(Attr("data-intersect", "true")));
        let matches = divs
            .filter_map(parse_matches)
            .flatten()
            .collect::<Vec<Match>>();

        Ok(matches)
    }
}

fn parse_matches(node: Node) -> Option<Vec<Match>> {
    println!("Parsing {}", node.index());

    let el_title = node.find(Class("ftbl__results-title")).next()?;
    let el_heading = el_title
        .find(Class("ftbl__results-title__heading"))
        .next()?;
    let x = el_heading.text(); // TODO: parse "Giornata n"
    println!("===> {}", x);

    // TODO: matches

    None
}
