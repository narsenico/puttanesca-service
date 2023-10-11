use super::{Match, MatchDay};

pub struct TestHunter;

impl super::Hunter for TestHunter {
    fn name(&self) -> String {
        String::from("Test Hunter")
    }

    fn find_matches(&self) -> crate::Result<Vec<Match>> {
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
