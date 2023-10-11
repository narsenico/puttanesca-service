use super::{Match, MatchDay};
use async_trait::async_trait;

pub struct TestHunter;

#[async_trait]
impl super::Hunter for TestHunter {
    fn name(&self) -> String {
        String::from("Test Hunter")
    }

    async fn find_matches(&self) -> crate::Result<Vec<Match>> {
        let matches = vec![Match {
            day: MatchDay {
                index: 0,
                date: "20230101".to_string(),
            },
            team1: "Blue".to_string(),
            team2: "Red".to_string(),
        }];

        Ok(matches)
    }
}
