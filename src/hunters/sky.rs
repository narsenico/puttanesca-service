use super::Match;
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

    async fn find_matches(&self) -> Result<Vec<Match>> {
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

#[derive(Debug, Default)]
struct PartialMatch {
    team1: String,
    team2: String,
    team1_goals: Option<usize>,
    team2_goals: Option<usize>,
}

impl PartialMatch {
    fn create_match(self, index: usize, date: &str) -> Match {
        Match {
            index,
            date: String::from(date),
            team1: self.team1,
            team2: self.team2,
            team1_goals: self.team1_goals,
            team2_goals: self.team2_goals,
        }
    }
}

fn parse_matches(node: Node) -> Option<Vec<Match>> {
    let el_title = node.find(Class("ftbl__results-title")).next()?;
    let index = extract_index(&el_title)?;

    let mut matches = Vec::<Match>::new();
    let el_table_rows =
        node.find(Class("ftbl__results-table").descendant(Name("tbody").descendant(Name("tr"))));
    let mut match_date: Option<String> = None;

    for row in el_table_rows {
        if row.is(Class("ftbl__match-data-row")) {
            match_date = extract_match_date(&row);
            continue;
        }

        if row.is(Class("ftbl__match-row")) {
            let md = match_date.as_ref()?;
            if let Some(partial_match) = extract_partial_match(&row) {
                let m = partial_match.create_match(index, md);
                matches.push(m);
            }
        }
    }

    Some(matches)
}

fn extract_index(node: &Node) -> Option<usize> {
    let el = node.find(Class("ftbl__results-title__heading")).next()?;
    el.text().split_once(' ')?.1.parse().ok()
}

fn extract_match_date(node: &Node) -> Option<String> {
    // Sabato 19 Ago 2023
    let el = node.find(Name("td").descendant(Name("span"))).next()?;
    let text = el.text();
    let tokens = text.split(' ').skip(1).collect::<Vec<&str>>();

    if tokens.len() != 3 {
        return None;
    }

    let month_index = get_month_index(tokens.get(1).unwrap())?;
    Some(format!(
        "{}-{:0>2}-{:0>2}",
        tokens.get(2).unwrap(),
        month_index,
        tokens.get(0).unwrap()
    ))
}

static MONTHS: [&str; 12] = [
    "Gen", "Feb", "Mar", "Apr", "Mag", "Giu", "Lug", "Ago", "Set", "Ott", "Nov", "Dic",
];

fn get_month_index(month: &str) -> Option<usize> {
    for (i, &m) in MONTHS.iter().enumerate() {
        if m == month {
            return Some(i);
        }
    }

    None
}

fn extract_partial_match(node: &Node) -> Option<PartialMatch> {
    let el_team1 = node.find(Class("ftbl__match-row__home")).next()?;
    let team1 = el_team1
        .find(Name("span").descendant(Name("span")))
        .skip(2)
        .next()?
        .text();

    let el_team2 = node.find(Class("ftbl__match-row__away")).next()?;
    let team2 = el_team2
        .find(Name("span").descendant(Name("span")))
        .skip(2)
        .next()?
        .text();

    let el_score = node.find(Class("ftbl__match-row__result")).next()?;
    let (team1_goals, team2_goals) = el_score
        .find(Name("span"))
        .next()?
        .text()
        .split_once(" - ")
        .map(|(g1, g2)| (g1.parse().ok(), g2.parse().ok()))
        .unwrap_or((None, None));

    Some(PartialMatch {
        team1,
        team2,
        team1_goals,
        team2_goals,
    })
}
