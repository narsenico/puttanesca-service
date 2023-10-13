use crate::error::Error;
use std::fmt::Display;

// TODO: manage date and time format (with struct?)

#[derive(Debug, Default, PartialEq)]
pub struct Match {
    pub(crate) match_day: usize,
    pub(crate) match_date: String,
    pub(crate) team1: String,
    pub(crate) team2: String,
    pub(crate) team1_score: Option<usize>,
    pub(crate) team2_score: Option<usize>,
}

impl Display for Match {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Match #{} {}: {} vs {} ({} - {})",
            self.match_day,
            self.match_date,
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
