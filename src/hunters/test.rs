use crate::{
    models::{Match, MatchDate},
    Result,
};

use async_trait::async_trait;

pub struct TestHunter;

#[async_trait]
impl super::Hunter for TestHunter {
    fn name(&self) -> String {
        String::from("Test Hunter")
    }

    async fn find_matches(&self) -> Result<Vec<Match>> {
        let matches = vec![Match {
            match_day: 0,
            match_date: MatchDate::from("2023-01-01").unwrap(),
            team1: "Blue".to_string(),
            team2: "Red".to_string(),
            team1_score: None,
            team2_score: None,
            team1_icon: None,
            team2_icon: None,
        }];

        Ok(matches)
    }
}
