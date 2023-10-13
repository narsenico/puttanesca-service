use crate::error::Error;
use std::fmt::Display;

// TODO: manage date and time format (with struct?)

#[derive(Debug, Default)]
pub struct Match {
    pub match_day: usize,
    pub date: String,
    pub team1: String,
    pub team2: String,
    pub team1_score: Option<usize>,
    pub team2_score: Option<usize>,
}

impl Display for Match {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Match #{} {}: {} vs {} ({} - {})",
            self.match_day,
            self.date,
            self.team1,
            self.team2,
            self.team1_score
                .map(|g| g.to_string())
                .unwrap_or("?".to_string()),
            self.team2_score
                .map(|g| g.to_string())
                .unwrap_or("?".to_string()),
        )
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
